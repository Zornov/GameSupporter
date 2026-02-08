mod capture;
mod mouse;

use std::time::Duration;
use anyhow::Result;
use capture::capture::Capture;
use capture::card::CardCapture;
use capture::ScreenSize;
use mouse::kmbox::KmBox;
use opencv::highgui;
use crate::mouse::mouse::Mouse;

const SCREEN_SIZE: ScreenSize = ScreenSize { width: 1920, height: 1080 };
const WINDOW_NAME: &str = "AiLocker";

fn get_card_index() -> Result<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let camera_index = input.trim().parse().unwrap_or(0);
    Ok(camera_index)
}
fn main() -> Result<()> {
    let mut kmbox = KmBox::connect("192.168.2.188", 61697, "FF313CAB")?;

    let camera_index = get_card_index()?;
    let mut card = CardCapture::new(camera_index, 420, SCREEN_SIZE)?;

    highgui::named_window(WINDOW_NAME, highgui::WINDOW_NORMAL)?;
    highgui::set_window_property(WINDOW_NAME, highgui::WND_PROP_TOPMOST, 1.0)?;
    highgui::resize_window(WINDOW_NAME, 420, 420)?;

    loop {
        let frame = card.grab()?;
        highgui::imshow(WINDOW_NAME, &frame)?;

        let key = highgui::wait_key(50)?;
        if key == 118 {
            kmbox.move_delta(0, -50);
            kmbox.right(true);
            kmbox.right(false);
            std::thread::sleep(Duration::from_millis(150));
            kmbox.left(true);
            kmbox.left(false);
        }
    }
}