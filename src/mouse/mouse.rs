

#[allow(dead_code)]
pub trait Mouse {
    fn left(&mut self, down: bool);
    fn right(&mut self, down: bool);

    fn move_delta(&mut self, x: i32, y: i32);
    // fn move_auto(&mut self, x: i16, y: i16, delay: u16);

    fn reboot(&mut self);
    fn shutdown(&mut self);
}