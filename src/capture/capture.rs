use anyhow::Result;
use opencv::core::Mat;

pub trait Capture {
    fn grab(&mut self) -> Result<Mat>;
}