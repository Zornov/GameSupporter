use crate::capture::capture::{Capture, ScreenSize};

pub struct ScreenCapture;

impl ScreenCapture {

    /// Create new screen capture.
    /// - `index`: monitor index (0 = primary)
    /// - `region`: region id or flag
    /// - `screen`: capture size/params
    pub fn new(index: i32, region: i32, screen: ScreenSize) -> Self {
        Self
    }
}

impl Capture for ScreenCapture {
    /// Grab a frame and return as `opencv::core::Mat`.
    fn grab(&mut self) -> anyhow::Result<opencv::core::Mat> {
        unimplemented!()
    }
}