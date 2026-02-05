use anyhow::Result;
use opencv::core::Mat;

/// Capture resolution parameters.
pub struct ScreenSize {
    /// Width of the capture area in pixels
    pub width: i32,
    /// Height of the capture area in pixels
    pub height: i32,
}

/// Trait for capturing frames from various sources.
///
/// Implement this trait to provide frame capture functionality
/// from different sources (screen, window, camera, etc.).
pub trait Capture {

    /// Captures a frame from the source.
    fn grab(&mut self) -> Result<Mat>;
}