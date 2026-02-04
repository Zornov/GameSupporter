mod capture;

use opencv::highgui;
use capture::capture::Capture;
use capture::card::CardCapture;
use anyhow::Result;
use crate::capture::card::ScreenSize;

const SCREEN_SIZE: ScreenSize = ScreenSize {
    width: 1920,
    height: 1080,
};
const WINDOW_NAME: &str = "Card Capture";

fn get_card_index() -> Result<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let camera_index = input.trim().parse().unwrap_or(0);
    Ok(camera_index)
}
fn main() -> Result<()> {
    let camera_index = get_card_index()?;
    let mut card = CardCapture::new(camera_index, 420, SCREEN_SIZE)?;

    highgui::named_window(WINDOW_NAME, highgui::WINDOW_NORMAL)?;
    highgui::set_window_property(WINDOW_NAME, highgui::WND_PROP_TOPMOST, 1.0)?;
    highgui::resize_window(WINDOW_NAME, 420, 420)?;

    loop {
        let frame = card.grab()?;
        highgui::imshow(WINDOW_NAME, &frame)?;

        let key = highgui::wait_key(30)?;
        if key == 27 {
            break;
        }
    }

    Ok(())
}
