use anyhow::{bail, Context, Result};
use opencv::{
    core::{Mat, Rect},
    prelude::*,
    videoio::{self, VideoCapture},
};

pub(crate) use super::capture::{Capture, ScreenSize};

pub struct CardCapture {
    cap: VideoCapture,
    roi: Rect,
}

impl CardCapture {
    pub fn new(index: i32, box_size: i32, screen_size: ScreenSize) -> Result<Self> {
        let mut cap = VideoCapture::new(index, videoio::CAP_DSHOW)
            .context("Failed to create VideoCapture")?;

        if !cap.is_opened()? {
            bail!("Unable to connect to capture card");
        }

        cap.set(videoio::CAP_PROP_FRAME_WIDTH, screen_size.width as f64)?;
        cap.set(videoio::CAP_PROP_FRAME_HEIGHT, screen_size.height as f64)?;

        let half = box_size / 2;

        let roi = Rect::new(
            screen_size.width / 2 - half,
            screen_size.height / 2 - half,
            box_size,
            box_size,
        );

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

        Mat::roi(&frame, self.roi)?.try_clone().map_err(Into::into)
    }
}