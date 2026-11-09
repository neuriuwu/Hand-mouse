# Hand Mouse ✨

### Your webcam, your hand, your cursor. Tiny computer-vision magic! ฅ^•ﻌ•^ฅ

[![Windows 11](https://img.shields.io/badge/Windows-11-0078D4?logo=windows)](https://www.microsoft.com/windows)
[![Rust](https://img.shields.io/badge/Rust-native-000000?logo=rust)](https://www.rust-lang.org/)
[![OpenCV DNN](https://img.shields.io/badge/AI-OpenCV%20DNN-5C3EE8?logo=opencv)](https://opencv.org/)

Hand Mouse is a native Rust webcam app that tracks a hand and steers the desktop cursor with your index fingertip. Bring your thumb and index finger together for a left click. No special mouse hardware required—just a webcam, a compatible ONNX model, and a little hand-powered wizardry (っ✧ω✧)っ

> **Model compatibility matters!** The live app expects a one-class hand-pose model with **21 landmarks** and a single output shaped `[1, 68, N]` or `[1, N, 68]`. The filename `hand_landmark.onnx` is just a name; a MediaPipe landmark-only model or a 17-keypoint COCO pose model is not a drop-in replacement.

## What this little hand-bot can do

- Track a hand from a live webcam feed with OpenCV DNN.
- Move the pointer using the index fingertip, with configurable smoothing to calm the jitters.
- Perform an edge-triggered left click when thumb and index finger pinch.
- Scroll the active window by holding a closed fist and moving the wrist vertically.
- Show a live preview with detected landmarks and pinch state.
- Run with clicking disabled for a safer tracking test.

**Not implemented yet:** right-click gestures and click-and-drag. Pinch performs a single left click; it does not hold the button down for a drag.

## Before the magic: Windows setup

You’ll need:

- Windows 11 (x64).
- Rust stable with the `x86_64-pc-windows-msvc` target.
- Visual Studio 2022 Build Tools or Visual Studio with the **Desktop development with C++** workload.
- vcpkg and OpenCV 4.x development libraries.
- LLVM/Clang with `libclang` available to the Rust OpenCV bindings.
- A webcam and a compatible hand-pose ONNX model.

### 1. Install the Rust toolchain

Install Rust with [rustup](https://rustup.rs/), then select the stable MSVC toolchain in PowerShell:

```powershell
rustup default stable-x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
```

### 2. Install OpenCV through vcpkg

If vcpkg is not installed yet, bootstrap it first. Skip the clone and bootstrap lines if it’s already installed, and use its existing directory in place of `C:\vcpkg` below.

```powershell
git clone https://github.com/microsoft/vcpkg C:\vcpkg
& 'C:\vcpkg\bootstrap-vcpkg.bat'
& 'C:\vcpkg\vcpkg.exe' install opencv4:x64-windows
```

In the same PowerShell session you’ll use to build, set the paths for vcpkg, OpenCV, and LLVM. Adjust these paths and the OpenCV version to match your installation:

```powershell
$env:VCPKG_ROOT = 'C:\vcpkg'
$env:OpenCV_DIR = "$env:VCPKG_ROOT\installed\x64-windows\share\opencv"
$env:OPENCV_VERSION = '4.14.0'
$env:LIBCLANG_PATH = 'C:\Program Files\LLVM\bin'
```

This checkout’s Cargo configuration already points at `C:\vcpkg` and OpenCV 4.14.0. If your tools are installed elsewhere, update those environment values/configuration accordingly. Build from a Visual Studio Developer PowerShell (or another shell with the MSVC tools available).

### 3. Add a compatible model

Place your model here:

```text
.\models\hand_landmark.onnx
```

The app does not download model weights. Use a model you have permission to use, and check its input/output contract before running it:

- Input: one RGB image tensor `[1, 3, S, S]`, normalized to `[0, 1]`.
- Output: one raw pose tensor `[1, 68, N]` or `[1, N, 68]`. At the default 640 input size, `N` is commonly `8400`.
- Per candidate: `cx, cy, width, height, hand_confidence`, then 21 `(x, y, landmark_confidence)` triplets.
- Landmarks use the familiar MediaPipe Hands ordering: thumb tip `4`, index fingertip `8`.

The app stretches the frame to a square input (no letterboxing), and maps landmark coordinates back to the camera frame. A model that uses different preprocessing or output channels needs an adapter.

## Quick start ✨

From the project directory, build the release binary:

```powershell
cargo build --release
```

Start with clicking disabled so you can check tracking first:

```powershell
.\target\release\hand-mouse.exe --model .\models\hand_landmark.onnx --no-click
```

When you’re ready, enable pinch-to-click:

```powershell
.\target\release\hand-mouse.exe --model .\models\hand_landmark.onnx
```

Press **Esc** in the preview to quit. With `--preview false`, use **Ctrl+C**. The app controls the system cursor, so start with `--no-click` and keep the preview available while tuning.

### Handy flags

| Flag | Default | What it does |
|---|---:|---|
| `--model <PATH>` | Required | Path to the ONNX hand-pose model. |
| `--camera <INDEX>` | `0` | Webcam device index. |
| `--input-size <PIXELS>` | `640` | Square model input width and height. |
| `--confidence <0..1>` | `0.35` | Minimum hand-candidate confidence. |
| `--pinch-threshold <VALUE>` | `0.045` | Thumb-index distance threshold, normalized by the frame diagonal. |
| `--cursor-deadzone <PIXELS>` | `2.5` | Index-tip movement deadband in camera pixels, applied before EMA filtering. |
| `--control-margin <0..0.5>` | `0.15` | Fraction excluded at each camera edge before aspect-ratio fitting. The active region maps to the display without stretching and is outlined in preview. |
| `--smoothing <0..1>` | `0.35` | EMA alpha for cursor movement. Lower values smooth more but add lag; higher values respond faster; `1` disables smoothing. |
| `--scroll-sensitivity <VALUE>` | `1.0` | Scroll-wheel response multiplier, from greater than `0` to `20`. |
| `--scroll-deadzone <PIXELS>` | `2.0` | Minimum wrist displacement from the last scroll anchor before scrolling. |
| `--mirror <true\|false>` | `true` | Mirror the camera image like a selfie view. |
| `--no-click` | Off | Move the cursor but do not emit left clicks. |
| `--preview <true\|false>` | `true` | Show or hide the camera preview. |

For example, use the central half of the camera view to span the display, while keeping movement smooth:

```powershell
.\target\release\hand-mouse.exe `
  --model .\models\hand_landmark.onnx `
  --control-margin 0.25 `
  --cursor-deadzone 2.5 `
  --smoothing 0.4 `
  --scroll-sensitivity 1.2 `
  --scroll-deadzone 2
```

The control rectangle is aspect-fitted to the display before it is mapped, so camera pixels are not stretched differently on each axis. Positions outside the active rectangle clamp to the nearest screen edge. Increase `--control-margin` to make the active zone smaller and reach screen edges with less hand travel; set it to `0` to start with the full camera frame (the aspect-ratio fit may still crop one axis). The index-tip filter uses a camera-pixel deadband before its EMA: movements within `--cursor-deadzone` are ignored, and larger movements have the deadband removed before smoothing. Lower `--smoothing` to suppress jitter more strongly; raise it for less lag.

Run `.\target\release\hand-mouse.exe --help` for the CLI help.

## Gesture guide ✋

| Hand action | Mouse action | Status |
|---|---|---|
| Move your hand / index fingertip | Move the cursor | Ready! |
| Pinch thumb tip and index fingertip | One left click when the pinch begins | Ready! |
| Open the pinch again | Re-arm the next left click | Ready! A small release margin prevents jittery repeat clicks. |
| Hold a closed fist; move your wrist down/up | Scroll the focused window down/up | Ready! The preview shows **SCROLL** while active. |
| Pinch and hold while moving | Click-and-drag | Not implemented yet |
| Make a right-click gesture | Right click | Not implemented yet |

The closed fist is recognized when the index, middle, ring, and pinky fingertips are all close to the wrist relative to their knuckles. The first fist frame establishes a wrist-position baseline; subsequent vertical wrist movement emits OS-level wheel events through Enigo. Scrolling pauses cursor movement until the gesture ends. `--no-click` disables pinch clicks but leaves scrolling enabled.

## Under the hood ⚙️

1. **Capture:** OpenCV reads webcam frames (requests 640×480) and optionally mirrors them.
2. **Prepare:** OpenCV `blob_from_image` resizes the frame to `--input-size`, swaps BGR to RGB, and scales pixel values by `1/255`.
3. **Infer:** The Rust `HandPoseModel` loads the ONNX graph with **OpenCV DNN**, configured for the OpenCV CPU backend. This project does not use ONNX Runtime.
4. **Decode:** The live hand path reads the 68-value candidate layout, filters by confidence, and selects the highest-confidence valid hand proposal. `src/vision.rs` also provides a standalone YOLO-pose parser with class-agnostic IoU NMS for callers that need multiple detections; the live cursor path currently uses the single-best-hand decoder.
5. **Control:** The index fingertip passes through a camera-pixel deadband and EMA filter, then maps from an aspect-fitted inset camera region to the primary display. Thumb-index distance drives left clicks; closed-fist wrist motion generates vertical Enigo wheel events with a deadzone and fractional-tick accumulator.

Inference runs one frame at a time on CPU; actual frame rate depends on your webcam, model, OpenCV build, and CPU. Camera availability and Windows input/security settings can also affect operation. The model weights are not bundled—bring a compatible model and check its license. (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧
