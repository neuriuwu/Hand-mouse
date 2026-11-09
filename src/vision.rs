use anyhow::{bail, Context, Result};
use nalgebra::Point2;
use opencv::{
    core::{Mat, Scalar, Size, CV_32F},
    dnn,
    prelude::*,
};
use std::path::Path;

pub const LANDMARK_COUNT: usize = 21; // Restored to 21 for the hand model
const VALUES_PER_DETECTION: usize = 5 + LANDMARK_COUNT * 3;
pub const YOLO_POSE_KEYPOINT_COUNT: usize = 21; // Restored to 21
const YOLO_POSE_CHANNEL_COUNT: usize = 5 + YOLO_POSE_KEYPOINT_COUNT * 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoseKeypoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct YoloPoseDetection {
    /// Bounding box edges in model-input pixel coordinates: left, top, right, bottom.
    pub bbox: [f32; 4],
    pub confidence: f32,
    pub keypoints: [PoseKeypoint; YOLO_POSE_KEYPOINT_COUNT],
}

/// Parses the single-image YOLOv8-pose output `[1, 68, N]`.
pub fn parse_yolov8_pose(
    output: &Mat,
    confidence_threshold: f32,
    nms_threshold: f32,
) -> Result<Vec<YoloPoseDetection>> {
    if output.dims() != 3 {
        bail!(
            "Expected a 3D YOLOv8-pose output tensor, got {} dimensions",
            output.dims()
        );
    }
    let shape = output.mat_size();
    let dimensions = [
        shape.get(0)? as usize,
        shape.get(1)? as usize,
        shape.get(2)? as usize,
    ];
    let owned_output = if output.is_continuous() {
        None
    } else {
        Some(output.try_clone()?)
    };
    let tensor = owned_output.as_ref().unwrap_or(output);
    parse_yolov8_pose_values(
        tensor.data_typed::<f32>()?,
        dimensions,
        confidence_threshold,
        nms_threshold,
    )
}

fn parse_yolov8_pose_values(
    values: &[f32],
    shape: [usize; 3],
    confidence_threshold: f32,
    nms_threshold: f32,
) -> Result<Vec<YoloPoseDetection>> {
    if shape[0] != 1 || shape[1] != YOLO_POSE_CHANNEL_COUNT || shape[2] == 0 {
        bail!(
            "Expected YOLOv8-pose output [1, {}, N] with N > 0, got shape {:?}",
            YOLO_POSE_CHANNEL_COUNT,
            shape
        );
    }
    if !confidence_threshold.is_finite() || !(0.0..=1.0).contains(&confidence_threshold) {
        bail!("Confidence threshold must be finite and between 0 and 1");
    }
    if !nms_threshold.is_finite() || !(0.0..=1.0).contains(&nms_threshold) {
        bail!("NMS threshold must be finite and between 0 and 1");
    }
    let candidate_count = shape[2];
    if values.len() != shape[0] * shape[1] * candidate_count {
        bail!(
            "ONNX output data length {} does not match tensor shape {:?}",
            values.len(),
            shape
        );
    }
    let value_at =
        |channel: usize, candidate: usize| -> f32 { values[channel * candidate_count + candidate] };

    let mut candidates = Vec::new();
    for candidate in 0..candidate_count {
        let confidence = value_at(4, candidate);
        let center_x = value_at(0, candidate);
        let center_y = value_at(1, candidate);
        let width = value_at(2, candidate);
        let height = value_at(3, candidate);
        if !confidence.is_finite()
            || confidence < confidence_threshold
            || !center_x.is_finite()
            || !center_y.is_finite()
            || !width.is_finite()
            || !height.is_finite()
            || width <= 0.0
            || height <= 0.0
        {
            continue;
        }

        let mut keypoints = [PoseKeypoint {
            x: 0.0,
            y: 0.0,
            confidence: 0.0,
        }; YOLO_POSE_KEYPOINT_COUNT];
        let mut valid_keypoints = true;
        for (index, keypoint) in keypoints.iter_mut().enumerate() {
            let channel = 5 + index * 3;
            *keypoint = PoseKeypoint {
                x: value_at(channel, candidate),
                y: value_at(channel + 1, candidate),
                confidence: value_at(channel + 2, candidate),
            };
            valid_keypoints &=
                keypoint.x.is_finite() && keypoint.y.is_finite() && keypoint.confidence.is_finite();
        }
        if !valid_keypoints {
            continue;
        }

        let bbox = [
            center_x - width / 2.0,
            center_y - height / 2.0,
            center_x + width / 2.0,
            center_y + height / 2.0,
        ];
        if bbox.iter().any(|coordinate| !coordinate.is_finite()) {
            continue;
        }
        candidates.push((confidence, bbox, keypoints));
    }

    candidates.sort_by(|left, right| right.0.total_cmp(&left.0));
    let mut kept: Vec<(f32, [f32; 4], [PoseKeypoint; YOLO_POSE_KEYPOINT_COUNT])> = Vec::new();
    for candidate in candidates {
        if kept
            .iter()
            .all(|selected| box_iou(candidate.1, selected.1) <= f64::from(nms_threshold))
        {
            kept.push(candidate);
        }
    }
    Ok(kept
        .into_iter()
        .map(|(confidence, bbox, keypoints)| YoloPoseDetection {
            bbox,
            confidence,
            keypoints,
        })
        .collect())
}

fn box_iou(left: [f32; 4], right: [f32; 4]) -> f64 {
    let left = left.map(f64::from);
    let right = right.map(f64::from);
    let intersection_width = (left[2].min(right[2]) - left[0].max(right[0])).max(0.0);
    let intersection_height = (left[3].min(right[3]) - left[1].max(right[1])).max(0.0);
    let intersection = intersection_width * intersection_height;
    let left_area = (left[2] - left[0]) * (left[3] - left[1]);
    let right_area = (right[2] - right[0]) * (right[3] - right[1]);
    let union = left_area + right_area - intersection;
    if union > 0.0 {
        intersection / union
    } else {
        0.0
    }
}

#[derive(Clone, Debug)]
pub struct HandPose {
    pub landmarks: [Point2<f32>; LANDMARK_COUNT],
    pub confidence: f32,
}

pub struct HandPoseModel {
    net: dnn::Net,
    input_size: i32,
    confidence_threshold: f32,
}

impl HandPoseModel {
    pub fn load(path: &Path, input_size: i32, confidence_threshold: f32) -> Result<Self> {
        if !path.is_file() {
            bail!("Model file does not exist: {}", path.display());
        }
        let path_string = path
            .to_str()
            .context("Model path is not valid UTF-8")?;
        let mut net = dnn::read_net_from_onnx(path_string)
            .with_context(|| format!("Could not load ONNX model {}", path.display()))?;
        net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;
        net.set_preferable_target(dnn::DNN_TARGET_CPU)?;
        Ok(Self {
            net,
            input_size,
            confidence_threshold,
        })
    }

    pub fn detect(&mut self, frame: &Mat) -> Result<Option<HandPose>> {
        let blob = dnn::blob_from_image(
            frame,
            1.0 / 255.0,
            Size::new(self.input_size, self.input_size),
            Scalar::default(),
            true,
            false,
            CV_32F,
        )?;
        self.net.set_input(&blob, "", 1.0, Scalar::default())?;
        let output = self.net.forward_single_def()?;
        decode_pose(
            &output,
            frame.cols() as f32,
            frame.rows() as f32,
            self.input_size as f32,
            self.confidence_threshold,
        )
    }
}

fn decode_pose(
    output: &Mat,
    frame_width: f32,
    frame_height: f32,
    model_size: f32,
    confidence_threshold: f32,
) -> Result<Option<HandPose>> {
    if output.dims() != 3 {
        bail!("Expected a 3D ONNX output tensor, got {} dimensions", output.dims());
    }
    let shape = output.mat_size();
    let dimensions = [
        shape.get(0)? as usize,
        shape.get(1)? as usize,
        shape.get(2)? as usize,
    ];
    let values = output.data_typed::<f32>()?;
    decode_values(
        values,
        dimensions,
        frame_width,
        frame_height,
        model_size,
        confidence_threshold,
    )
}

fn decode_values(
    values: &[f32],
    shape: [usize; 3],
    frame_width: f32,
    frame_height: f32,
    model_size: f32,
    confidence_threshold: f32,
) -> Result<Option<HandPose>> {
    let channels_first = shape[1] == VALUES_PER_DETECTION && shape[2] != VALUES_PER_DETECTION;
    let channels_last = shape[2] == VALUES_PER_DETECTION && shape[1] != VALUES_PER_DETECTION;
    if !channels_first && !channels_last {
        bail!(
            "Expected output [1, 68, N] or [1, N, 68], got shape {:?}",
            shape
        );
    }
    if shape[0] != 1 {
        bail!("Expected a single-image model output, got batch size {}", shape[0]);
    }
    let candidate_count = if channels_first { shape[2] } else { shape[1] };
    if candidate_count == 0 || values.len() != shape[0] * shape[1] * shape[2] {
        bail!(
            "ONNX output data length {} does not match tensor shape {:?}",
            values.len(),
            shape
        );
    }
    let channel_count = if channels_first { shape[1] } else { shape[2] };
    if channel_count != VALUES_PER_DETECTION {
        bail!(
            "Expected {} values per detection (box, confidence, landmarks); got {}",
            VALUES_PER_DETECTION,
            channel_count
        );
    }
    let value_at = |candidate: usize, channel: usize| -> f32 {
        if channels_first {
            values[channel * candidate_count + candidate]
        } else {
            values[candidate * VALUES_PER_DETECTION + channel]
        }
    };

    let mut best: Option<(usize, f32)> = None;
    for candidate in 0..candidate_count {
        let confidence = value_at(candidate, 4);
        let valid_landmarks = (0..LANDMARK_COUNT).all(|landmark| {
            let offset = 5 + landmark * 3;
            value_at(candidate, offset).is_finite()
                && value_at(candidate, offset + 1).is_finite()
        });
        if valid_landmarks
            && confidence.is_finite()
            && confidence >= confidence_threshold
            && best.map_or(true, |(_, best_confidence)| confidence > best_confidence)
        {
            best = Some((candidate, confidence));
        }
    }

    let Some((candidate, confidence)) = best else {
        return Ok(None);
    };
    let scale_x = frame_width / model_size;
    let scale_y = frame_height / model_size;
    let landmarks = std::array::from_fn(|landmark| {
        let offset = 5 + landmark * 3;
        Point2::new(
            value_at(candidate, offset) * scale_x,
            value_at(candidate, offset + 1) * scale_y,
        )
    });
    Ok(Some(HandPose {
        landmarks,
        confidence,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_yolov8_pose_and_suppresses_overlapping_boxes() {
        let candidate_count = 3;
        let mut values = vec![0.0; YOLO_POSE_CHANNEL_COUNT * candidate_count];
        let set = |values: &mut [f32], channel: usize, candidate: usize, value: f32| {
            values[channel * candidate_count + candidate] = value;
        };
        for candidate in 0..candidate_count {
            set(&mut values, 0, candidate, 100.0 + candidate as f32 * 2.0);
            set(&mut values, 1, candidate, 100.0);
            set(&mut values, 2, candidate, 80.0);
            set(&mut values, 3, candidate, 80.0);
            set(&mut values, 4, candidate, [0.9, 0.8, 0.2][candidate]);
            for keypoint in 0..YOLO_POSE_KEYPOINT_COUNT {
                let channel = 5 + keypoint * 3;
                set(&mut values, channel, candidate, keypoint as f32);
                set(&mut values, channel + 1, candidate, keypoint as f32 + 1.0);
                set(&mut values, channel + 2, candidate, 0.95);
            }
        }

        let detections = parse_yolov8_pose_values(
            &values,
            [1, YOLO_POSE_CHANNEL_COUNT, candidate_count],
            0.5,
            0.5,
        )
        .unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].confidence, 0.9);
        assert_eq!(detections[0].bbox, [60.0, 60.0, 140.0, 140.0]);
    }

    #[test]
    fn rejects_unexpected_yolov8_pose_shapes() {
        assert!(parse_yolov8_pose_values(&[], [1, 68, 0], 0.5, 0.5).is_err());
        assert!(parse_yolov8_pose_values(&[], [1, 69, 8400], 0.5, 0.5).is_err());
    }

    #[test]
    fn decodes_channels_first_hand_landmarks_and_scales_to_frame() {
        let mut values = vec![0.0; VALUES_PER_DETECTION];
        values[4] = 0.9;
        values[5 + 8 * 3] = 320.0;
        values[5 + 8 * 3 + 1] = 160.0;
        let pose = decode_values(&values, [1, VALUES_PER_DETECTION, 1], 640.0, 480.0, 640.0, 0.5)
            .unwrap()
            .unwrap();
        assert_eq!(pose.landmarks[8], Point2::new(320.0, 120.0));
        assert_eq!(pose.confidence, 0.9);
    }
}