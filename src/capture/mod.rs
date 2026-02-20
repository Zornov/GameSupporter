use anyhow::Result;
use opencv::core::Mat;

pub mod card;

pub use card::CardCapture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

impl ScreenSize {
    pub fn new(width: i32, height: i32) -> Result<Self> {
        anyhow::ensure!(width > 0 && height > 0, "ScreenSize must be positive");
        Ok(Self { width, height })
    }
}

pub trait Capture {
    fn grab(&mut self) -> Result<Mat>;
}