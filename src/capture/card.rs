use anyhow::{bail, ensure, Context, Result};
use opencv::{
    core::{Mat, Rect},
    prelude::*,
    videoio::{self, VideoCapture},
};
use crate::capture::{Capture, ScreenSize};

pub struct CardCapture {
    cap: VideoCapture,
    roi: Rect,
}

impl CardCapture {
    pub fn new(index: i32, region: i32, screen: ScreenSize) -> Result<Self> {
        ensure!(region > 0, "region must be positive");

        let mut cap = VideoCapture::new(index, videoio::CAP_DSHOW)
            .context("Failed to create VideoCapture")?;

        if !cap.is_opened()? {
            bail!("Unable to connect to capture card");
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, screen.width as f64)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, screen.height as f64)?;

        let half = region / 2;
        let x = screen.width / 2 - half;
        let y = screen.height / 2 - half;

        ensure!(x >= 0 && y >= 0, "ROI starts outside frame");
        ensure!(
            x + region <= screen.width && y + region <= screen.height,
            "ROI exceeds frame bounds"
        );

        let roi = Rect::new(x, y, region, region);

        Ok(Self { cap, roi })
    }
}

impl Capture for CardCapture {
    fn grab(&mut self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.cap.read(&mut frame)?;

        if frame.empty() {
            bail!("Empty frame");
        }

        Ok(Mat::roi(&frame, self.roi)?.try_clone()?)
    }
}