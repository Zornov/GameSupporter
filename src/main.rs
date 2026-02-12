mod capture;
mod mouse;

use crate::mouse::mouse::Mouse;
use anyhow::Result;
use capture::capture::Capture;
use capture::card::CardCapture;
use capture::ScreenSize;
use mouse::kmbox::KmBox;
use opencv::highgui;
use ort::ep::{DirectML, CUDA};
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;

const SCREEN_SIZE: ScreenSize = ScreenSize { width: 1920, height: 1080 };
const WINDOW_NAME: &str = "AiLocker";

fn get_card_index() -> Result<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let camera_index = input.trim().parse().unwrap_or(0);
    Ok(camera_index)
}

fn setup_ml() -> Result<()> {
    let mut model = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .with_execution_providers([
            CUDA::default().build(),
            DirectML::default().build()
        ])?
        .commit_from_file("yolo.onnx")?;
    Ok(())
}
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    setup_ml();

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
            kmbox.set_config("192.168.2.188".parse()?, 2000)?;
        }
    }
}