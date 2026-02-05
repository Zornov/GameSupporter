use anyhow::{bail, Context, Result};
use opencv::{
    core::{Mat, Rect},
    prelude::*,
    videoio::{self, VideoCapture},
};

use super::capture::{Capture, ScreenSize};

/// Card capture backed by OpenCV `VideoCapture`.
/// Wraps a capture device and a region of interest (ROI).
pub struct CardCapture {
    /// OpenCV video capture handle.
    cap: VideoCapture,
    /// Region of interest inside captured frames.
    roi: Rect,
}

impl CardCapture {

    /// Create a new `CardCapture`.
    /// - `index`: device index for `VideoCapture`.
    /// - `region`: size (width/height) of the square ROI.
    /// - `screen`: desired capture resolution (`ScreenSize`).
    pub fn new(index: i32, region: i32, screen: ScreenSize) -> Result<Self> {
        let mut cap = VideoCapture::new(index, videoio::CAP_DSHOW)
            .context("Failed to create VideoCapture")?;

        if !cap.is_opened()? {
            bail!("Unable to connect to capture card");
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, screen.width as f64)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, screen.height as f64)?;

        let half = region / 2;

        let roi = Rect::new(
            screen.width / 2 - half,
            screen.height / 2 - half,
            region,
            region
        );

        Ok(Self { cap, roi })
    }
}

impl Capture for CardCapture {

    /// Grab a frame from the capture device and return the ROI as `opencv::core::Mat`.
    /// Returns an error on read failure or empty frame.
    fn grab(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;

        if frame.empty() {
            bail!("Empty frame");
        }

        Mat::roi(&frame, self.roi)?.try_clone().map_err(Into::into)
    }
}