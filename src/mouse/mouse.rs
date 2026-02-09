/// High-level interface for controlling the mouse.
///
/// Can be implemented by different backends (e.g., KmBox, RDP) to provide a global API.
#[allow(dead_code)]
pub trait Mouse {

    /// Press or release the left mouse button.
    fn left(&mut self, down: bool);

    /// Press or release the right mouse button.
    fn right(&mut self, down: bool);


    /// Move the mouse cursor by a relative delta (x, y) without interpolation(delay).
    fn move_delta(&mut self, x: i32, y: i32);

    /// Move the mouse cursor to an absolute position (x, y) with interpolation over a specified delay in milliseconds.
    fn move_auto(&mut self, x: i32, y: i32, delay: u32);


    /// Reboot the remote device.
    fn reboot(&mut self);

    /// Shutdown the remote device.
    fn shutdown(&mut self);
}