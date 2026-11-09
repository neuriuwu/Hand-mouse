mod mouse;
mod vision;

use anyhow::{bail, Context, Result};
use clap::Parser;
use enigo::{Enigo, Settings};
use opencv::{
    core::{self, Mat, Point, Scalar},
    highgui, imgproc,
    prelude::*,
    videoio,
};
use std::{path::PathBuf, thread, time::Duration};

use crate::{
    mouse::{MouseController, MAX_SCROLL_SENSITIVITY},
    vision::HandPoseModel,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Path to a YOLO-style, one-class hand-pose ONNX model with 21 landmarks.
    #[arg(long)]
    model: PathBuf,

    /// Webcam device index.
    #[arg(long, default_value_t = 0)]
    camera: i32,

    /// Model input width and height in pixels.
    #[arg(long, default_value_t = 640)]
    input_size: i32,

    /// Minimum hand detection confidence.
    #[arg(long, default_value_t = 0.35)]
    confidence: f32,

    /// Normalized thumb-index distance that starts a click.
    #[arg(long, default_value_t = 0.045)]
    pinch_threshold: f32,

    /// EMA alpha in (0, 1]; lower values smooth more, and 1 disables smoothing.
    #[arg(long, default_value_t = 0.35)]
    smoothing: f32,

    /// Camera-pixel deadzone for index-tip jitter before EMA filtering.
    #[arg(long, default_value_t = 2.5)]
    cursor_deadzone: f32,

    /// Camera-frame margin excluded from cursor control on each edge (0.0 to <0.5).
    #[arg(long, default_value_t = 0.15)]
    control_margin: f32,

    /// Scroll response multiplier in (0, 20].
    #[arg(long, default_value_t = 1.0)]
    scroll_sensitivity: f32,

    /// Minimum wrist movement from the last scroll anchor, in camera pixels.
    #[arg(long, default_value_t = 2.0)]
    scroll_deadzone: f32,

    /// Mirror the webcam image, as in a selfie preview.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    mirror: bool,

    /// Move the cursor without generating clicks.
    #[arg(long)]
    no_click: bool,

    /// Show the camera preview. Press Esc in the preview to quit.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    preview: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.input_size <= 0 {
        bail!("--input-size must be greater than zero");
    }
    if !(0.0..=1.0).contains(&args.confidence) {
        bail!("--confidence must be between 0 and 1");
    }
    if !args.pinch_threshold.is_finite() || args.pinch_threshold <= 0.0 {
        bail!("--pinch-threshold must be a finite value greater than zero");
    }
    if !(0.0..=1.0).contains(&args.smoothing) || args.smoothing == 0.0 {
        bail!("--smoothing must be in (0, 1]");
    }
    if !args.cursor_deadzone.is_finite() || args.cursor_deadzone < 0.0 {
        bail!("--cursor-deadzone must be finite and non-negative");
    }
    if !args.control_margin.is_finite() || !(0.0..0.5).contains(&args.control_margin) {
        bail!("--control-margin must be finite and in [0, 0.5)");
    }
    if !args.scroll_sensitivity.is_finite()
        || !(0.0..=MAX_SCROLL_SENSITIVITY).contains(&args.scroll_sensitivity)
        || args.scroll_sensitivity == 0.0
    {
        bail!(
            "--scroll-sensitivity must be in (0, {}]",
            MAX_SCROLL_SENSITIVITY
        );
    }
    if !args.scroll_deadzone.is_finite() || args.scroll_deadzone < 0.0 {
        bail!("--scroll-deadzone must be finite and non-negative");
    }

    let mut model = HandPoseModel::load(&args.model, args.input_size, args.confidence)?;
    let mut camera = videoio::VideoCapture::new(args.camera, videoio::CAP_ANY)
        .context("Failed to initialize the webcam")?;
    if !camera.is_opened()? {
        bail!("Could not open webcam at index {}", args.camera);
    }
    camera.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    camera.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;

    let enigo = Enigo::new(&Settings::default()).context("Failed to initialize mouse control")?;
    let mut mouse = MouseController::new(
        enigo,
        args.smoothing,
        args.cursor_deadzone,
        args.control_margin,
        args.scroll_sensitivity,
        args.scroll_deadzone,
        args.pinch_threshold,
        args.no_click,
    )?;
    if args.preview {
        highgui::named_window("Hand Mouse - Esc to quit", highgui::WINDOW_AUTOSIZE)?;
    }

    let mut frame = Mat::default();
    let mut working = Mat::default();
    loop {
        if !camera.read(&mut frame)? || frame.empty() {
            bail!("The webcam stopped returning frames");
        }
        if args.mirror {
            core::flip(&frame, &mut working, 1)?;
        } else {
            frame.copy_to(&mut working)?;
        }

        if let Some(pose) = model.detect(&working)? {
            mouse.update(&pose, working.cols(), working.rows())?;
            if args.preview {
                let control_bounds = mouse.control_bounds(working.cols(), working.rows());
                draw_pose(
                    &mut working,
                    &pose,
                    mouse.is_pinched(),
                    mouse.is_scrolling(),
                    control_bounds,
                )?;
            }
        } else {
            mouse.hand_lost();
        }

        if args.preview {
            highgui::imshow("Hand Mouse - Esc to quit", &working)?;
            if highgui::wait_key(1)? & 0xff == 27 {
                break;
            }
        } else {
            // Avoid busy-spinning if a camera/backend yields frames faster than it captures.
            thread::sleep(Duration::from_millis(1));
        }
    }
    Ok(())
}

fn draw_pose(
    frame: &mut Mat,
    pose: &vision::HandPose,
    pinched: bool,
    scrolling: bool,
    control_bounds: [f32; 4],
) -> Result<()> {
    let color = if scrolling {
        Scalar::new(255.0, 180.0, 0.0, 0.0)
    } else if pinched {
        Scalar::new(0.0, 0.0, 255.0, 0.0)
    } else {
        Scalar::new(0.0, 255.0, 0.0, 0.0)
    };
    let control_rect = opencv::core::Rect::new(
        control_bounds[0].round() as i32,
        control_bounds[1].round() as i32,
        (control_bounds[2] - control_bounds[0]).round() as i32,
        (control_bounds[3] - control_bounds[1]).round() as i32,
    );
    imgproc::rectangle(
        frame,
        control_rect,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        1,
        imgproc::LINE_AA,
        0,
    )?;
    for (index, point) in pose.landmarks.iter().enumerate() {
        let center = Point::new(point.x.round() as i32, point.y.round() as i32);
        imgproc::circle(
            frame,
            center,
            4,
            color,
            imgproc::FILLED,
            imgproc::LINE_AA,
            0,
        )?;
        if index == 4 || index == 8 {
            imgproc::put_text(
                frame,
                if index == 4 { "thumb" } else { "index" },
                Point::new(center.x + 5, center.y - 5),
                imgproc::FONT_HERSHEY_SIMPLEX,
                0.45,
                color,
                1,
                imgproc::LINE_AA,
                false,
            )?;
        }
    }
    if scrolling {
        imgproc::put_text(
            frame,
            "SCROLL",
            Point::new(12, 28),
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.8,
            color,
            2,
            imgproc::LINE_AA,
            false,
        )?;
    }
    Ok(())
}
