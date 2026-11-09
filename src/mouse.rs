use anyhow::Result;
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Mouse};
use nalgebra::Point2;

use crate::vision::HandPose;

const SCROLL_PIXELS_PER_TICK: f32 = 12.0;
const MAX_SCROLL_DELTA_PER_FRAME: f32 = 120.0;
const MAX_SCROLL_TICKS_PER_FRAME: i32 = 12;
const FIST_CURL_RATIO: f32 = 1.2;
const FIST_CURL_MARGIN_PALM_RATIO: f32 = 0.1;
pub const MAX_SCROLL_SENSITIVITY: f32 = 20.0;

pub struct MouseController {
    enigo: Enigo,
    screen_width: u32,
    screen_height: u32,
    smoothing: f32,
    cursor_deadzone: f32,
    control_margin: f32,
    scroll_sensitivity: f32,
    scroll_deadzone: f32,
    pinch_threshold: f32,
    smoothed_camera_position: Option<Point2<f32>>,
    previous_fist_y: Option<f32>,
    scroll_remainder: f32,
    pinched: bool,
    scrolling: bool,
    no_click: bool,
}

impl MouseController {
    pub fn new(
        enigo: Enigo,
        smoothing: f32,
        cursor_deadzone: f32,
        control_margin: f32,
        scroll_sensitivity: f32,
        scroll_deadzone: f32,
        pinch_threshold: f32,
        no_click: bool,
    ) -> Result<Self> {
        anyhow::ensure!(
            smoothing.is_finite() && (0.0..=1.0).contains(&smoothing) && smoothing > 0.0,
            "Smoothing must be finite and in (0, 1]"
        );
        anyhow::ensure!(
            cursor_deadzone.is_finite() && cursor_deadzone >= 0.0,
            "Cursor deadzone must be finite and non-negative"
        );
        anyhow::ensure!(
            control_margin.is_finite() && (0.0..0.5).contains(&control_margin),
            "Control margin must be finite and in [0, 0.5)"
        );
        anyhow::ensure!(
            scroll_sensitivity.is_finite()
                && scroll_sensitivity > 0.0
                && scroll_sensitivity <= MAX_SCROLL_SENSITIVITY,
            "Scroll sensitivity must be finite and in (0, 20]"
        );
        anyhow::ensure!(
            scroll_deadzone.is_finite() && scroll_deadzone >= 0.0,
            "Scroll deadzone must be finite and non-negative"
        );
        let (screen_width, screen_height) = enigo.main_display()?;
        anyhow::ensure!(
            screen_width > 0 && screen_height > 0,
            "The primary display reported an invalid size"
        );
        Ok(Self {
            enigo,
            screen_width: screen_width as u32,
            screen_height: screen_height as u32,
            smoothing,
            cursor_deadzone,
            control_margin,
            scroll_sensitivity,
            scroll_deadzone,
            pinch_threshold,
            smoothed_camera_position: None,
            previous_fist_y: None,
            scroll_remainder: 0.0,
            pinched: false,
            scrolling: false,
            no_click,
        })
    }

    pub fn update(&mut self, pose: &HandPose, frame_width: i32, frame_height: i32) -> Result<()> {
        anyhow::ensure!(
            frame_width > 0 && frame_height > 0,
            "Camera frame dimensions must be greater than zero"
        );
        let index_tip = pose.landmarks[8];
        let thumb_tip = pose.landmarks[4];
        if is_closed_fist(pose) {
            let wrist_y = pose.landmarks[0].y;
            if let Some(previous_y) = self.previous_fist_y {
                if let Some(delta_y) =
                    scroll_delta_after_deadzone(wrist_y, previous_y, self.scroll_deadzone)
                {
                    self.scroll_by_delta(delta_y)?;
                    self.previous_fist_y = Some(wrist_y);
                }
            } else {
                self.previous_fist_y = Some(wrist_y);
            }
            self.smoothed_camera_position = None;
            self.pinched = false;
            self.scrolling = true;
            return Ok(());
        }
        if self.scrolling {
            self.smoothed_camera_position = None;
            self.previous_fist_y = None;
            self.scroll_remainder = 0.0;
        }
        self.scrolling = false;

        let camera_target = Point2::new(index_tip.x, index_tip.y);
        anyhow::ensure!(
            camera_target.x.is_finite() && camera_target.y.is_finite(),
            "Index fingertip coordinates must be finite"
        );
        let filtered_position = filter_cursor_position(
            camera_target,
            self.smoothed_camera_position,
            self.smoothing,
            self.cursor_deadzone,
        );
        self.smoothed_camera_position = Some(filtered_position);
        let bounds = control_bounds(
            frame_width as f32,
            frame_height as f32,
            self.screen_width as f32,
            self.screen_height as f32,
            self.control_margin,
        );
        let position = map_camera_to_screen(filtered_position, bounds);

        let max_x = self.screen_width.saturating_sub(1) as f32;
        let max_y = self.screen_height.saturating_sub(1) as f32;
        self.enigo.move_mouse(
            (position.x.clamp(0.0, 1.0) * max_x).round() as i32,
            (position.y.clamp(0.0, 1.0) * max_y).round() as i32,
            Coordinate::Abs,
        )?;

        let diagonal = (frame_width as f32).hypot(frame_height as f32);
        let pinch_distance = (thumb_tip - index_tip).norm() / diagonal;
        let release_threshold = self.pinch_threshold * 1.35;
        if pinch_distance < self.pinch_threshold {
            if !self.pinched && !self.no_click {
                self.enigo.button(Button::Left, Direction::Click)?;
            }
            self.pinched = true;
        } else if pinch_distance > release_threshold {
            self.pinched = false;
        }
        Ok(())
    }

    pub fn control_bounds(&self, frame_width: i32, frame_height: i32) -> [f32; 4] {
        control_bounds(
            frame_width as f32,
            frame_height as f32,
            self.screen_width as f32,
            self.screen_height as f32,
            self.control_margin,
        )
    }

    fn scroll_by_delta(&mut self, delta_y: f32) -> Result<()> {
        let ticks =
            scroll_ticks_for_delta(delta_y, self.scroll_sensitivity, &mut self.scroll_remainder);
        if ticks != 0 {
            self.enigo.scroll(ticks, Axis::Vertical)?;
        }
        Ok(())
    }

    pub fn hand_lost(&mut self) {
        self.smoothed_camera_position = None;
        self.previous_fist_y = None;
        self.scroll_remainder = 0.0;
        self.pinched = false;
        self.scrolling = false;
    }

    pub fn is_pinched(&self) -> bool {
        self.pinched
    }

    pub fn is_scrolling(&self) -> bool {
        self.scrolling
    }
}

fn control_bounds(
    frame_width: f32,
    frame_height: f32,
    screen_width: f32,
    screen_height: f32,
    margin: f32,
) -> [f32; 4] {
    let mut left = frame_width * margin;
    let mut top = frame_height * margin;
    let mut right = frame_width * (1.0 - margin);
    let mut bottom = frame_height * (1.0 - margin);
    let target_aspect = screen_width / screen_height;
    let region_aspect = (right - left) / (bottom - top);
    if region_aspect > target_aspect {
        let width = (bottom - top) * target_aspect;
        left = (left + right - width) * 0.5;
        right = left + width;
    } else {
        let height = (right - left) / target_aspect;
        top = (top + bottom - height) * 0.5;
        bottom = top + height;
    }
    [left, top, right, bottom]
}

fn map_camera_to_screen(position: Point2<f32>, bounds: [f32; 4]) -> Point2<f32> {
    Point2::new(
        ((position.x - bounds[0]) / (bounds[2] - bounds[0])).clamp(0.0, 1.0),
        ((position.y - bounds[1]) / (bounds[3] - bounds[1])).clamp(0.0, 1.0),
    )
}

fn filter_cursor_position(
    target: Point2<f32>,
    previous: Option<Point2<f32>>,
    alpha: f32,
    deadzone: f32,
) -> Point2<f32> {
    let Some(previous) = previous else {
        return target;
    };
    let delta = target - previous;
    let distance = delta.norm();
    if !distance.is_finite() || distance <= deadzone {
        return previous;
    }
    let deadzone_adjusted_delta = delta * ((distance - deadzone) / distance);
    previous + deadzone_adjusted_delta * alpha
}

fn landmark_distance(pose: &HandPose, first: usize, second: usize) -> f32 {
    (pose.landmarks[first] - pose.landmarks[second]).norm()
}

fn scroll_ticks_for_delta(delta_y: f32, sensitivity: f32, remainder: &mut f32) -> i32 {
    if !delta_y.is_finite() {
        return 0;
    }
    let bounded_delta = delta_y.clamp(-MAX_SCROLL_DELTA_PER_FRAME, MAX_SCROLL_DELTA_PER_FRAME);
    *remainder += bounded_delta * sensitivity / SCROLL_PIXELS_PER_TICK;
    let accumulated_ticks = remainder.trunc() as i32;
    let ticks = accumulated_ticks.clamp(-MAX_SCROLL_TICKS_PER_FRAME, MAX_SCROLL_TICKS_PER_FRAME);
    *remainder = if ticks == accumulated_ticks {
        *remainder - ticks as f32
    } else {
        0.0
    };
    ticks
}

fn scroll_delta_after_deadzone(current_y: f32, anchor_y: f32, deadzone: f32) -> Option<f32> {
    let delta_y = current_y - anchor_y;
    (delta_y.is_finite() && delta_y.abs() >= deadzone).then_some(delta_y)
}

fn is_closed_fist(pose: &HandPose) -> bool {
    let palm_size = landmark_distance(pose, 5, 17);
    if !palm_size.is_finite() || palm_size <= f32::EPSILON {
        return false;
    }
    if pose
        .landmarks
        .iter()
        .any(|point| !point.coords.iter().all(|v| v.is_finite()))
    {
        return false;
    }

    [(5, 8), (9, 12), (13, 16), (17, 20)]
        .iter()
        .all(|(knuckle, tip)| {
            landmark_distance(pose, *tip, 0)
                < landmark_distance(pose, *knuckle, 0) + palm_size * FIST_CURL_MARGIN_PALM_RATIO
                && landmark_distance(pose, *tip, 0)
                    < landmark_distance(pose, *knuckle, 0) * FIST_CURL_RATIO
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_region_preserves_screen_aspect_ratio() {
        let bounds = control_bounds(640.0, 480.0, 1920.0, 1080.0, 0.0);
        assert_eq!(bounds, [0.0, 60.0, 640.0, 420.0]);
        assert_eq!(
            map_camera_to_screen(Point2::new(0.0, 60.0), bounds),
            Point2::new(0.0, 0.0)
        );
        assert_eq!(
            map_camera_to_screen(Point2::new(640.0, 420.0), bounds),
            Point2::new(1.0, 1.0)
        );
        assert_eq!(
            map_camera_to_screen(Point2::new(320.0, 240.0), bounds),
            Point2::new(0.5, 0.5)
        );
    }

    #[test]
    fn inset_control_region_maps_its_edges_to_screen_edges() {
        let bounds = control_bounds(640.0, 480.0, 1920.0, 1080.0, 0.1);
        assert_eq!(
            map_camera_to_screen(Point2::new(bounds[0], bounds[1]), bounds),
            Point2::new(0.0, 0.0)
        );
        assert_eq!(
            map_camera_to_screen(Point2::new(bounds[2], bounds[3]), bounds),
            Point2::new(1.0, 1.0)
        );
        assert_eq!(
            map_camera_to_screen(Point2::new(0.0, 0.0), bounds),
            Point2::new(0.0, 0.0)
        );
    }

    #[test]
    fn scroll_accumulator_preserves_fractional_wheel_motion() {
        let mut remainder = 0.0;
        for _ in 0..3 {
            assert_eq!(scroll_ticks_for_delta(3.0, 1.0, &mut remainder), 0);
        }
        assert_eq!(scroll_ticks_for_delta(3.0, 1.0, &mut remainder), 1);
        assert!(remainder.abs() < f32::EPSILON);
    }

    #[test]
    fn scroll_ticks_follow_vertical_hand_motion() {
        let mut remainder = 0.0;
        assert_eq!(scroll_ticks_for_delta(24.0, 1.0, &mut remainder), 2);
        assert_eq!(scroll_ticks_for_delta(-12.0, 1.0, &mut remainder), -1);
    }

    #[test]
    fn scroll_deadzone_accumulates_small_hand_movements_from_its_anchor() {
        let anchor = 100.0;
        assert_eq!(scroll_delta_after_deadzone(101.0, anchor, 2.0), None);
        assert_eq!(scroll_delta_after_deadzone(102.0, anchor, 2.0), Some(2.0));
    }

    #[test]
    fn detects_closed_fist_and_rejects_an_extended_fingertip() {
        let mut landmarks = [Point2::new(0.0, 0.0); 21];
        landmarks[0] = Point2::new(100.0, 200.0);
        landmarks[5] = Point2::new(80.0, 140.0);
        landmarks[8] = Point2::new(85.0, 160.0);
        landmarks[9] = Point2::new(100.0, 135.0);
        landmarks[12] = Point2::new(102.0, 160.0);
        landmarks[13] = Point2::new(120.0, 140.0);
        landmarks[16] = Point2::new(115.0, 170.0);
        landmarks[17] = Point2::new(140.0, 150.0);
        landmarks[20] = Point2::new(135.0, 175.0);
        let pose = HandPose {
            landmarks,
            confidence: 1.0,
        };
        assert!(is_closed_fist(&pose));

        let mut index_extended = pose.clone();
        index_extended.landmarks[8] = Point2::new(80.0, 80.0);
        assert!(!is_closed_fist(&index_extended));
    }

    #[test]
    fn cursor_filter_rejects_micro_jitter_and_smooths_larger_moves() {
        let previous = Point2::new(100.0, 100.0);
        assert_eq!(
            filter_cursor_position(Point2::new(101.0, 100.0), Some(previous), 0.5, 2.0),
            previous
        );
        assert_eq!(
            filter_cursor_position(Point2::new(110.0, 100.0), Some(previous), 0.5, 2.0),
            Point2::new(104.0, 100.0)
        );
    }

    #[test]
    fn cursor_filter_initializes_at_the_first_observation() {
        let target = Point2::new(12.0, 34.0);
        assert_eq!(filter_cursor_position(target, None, 0.35, 2.5), target);
    }
}
