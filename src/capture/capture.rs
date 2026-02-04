use anyhow::Result;
use opencv::core::Mat;

pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

pub trait Capture {
    fn grab(&mut self) -> Result<Mat>;
}