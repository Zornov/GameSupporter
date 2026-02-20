mod capture;
mod mouse;

use anyhow::Result;
use opencv::highgui;
use ort::{
    ep::{CUDA, DirectML, TensorRT},
    session::{
        builder::GraphOptimizationLevel,
        Session,
    },
};

use crate::capture::{Capture, CardCapture, ScreenSize};

const SCREEN_SIZE: ScreenSize = ScreenSize { width: 1920, height: 1080 };
const WINDOW_NAME: &str = "AiLocker";

fn read_card_index() -> Result<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s.trim().parse().unwrap_or(0))
}

fn setup_ml() -> Result<Session> {
    Session::builder()?
        .with_execution_providers([
            TensorRT::default().build(),
            CUDA::default().build(),
            DirectML::default().build(),
        ])?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file("model.onnx")
        .map_err(Into::into)
}

fn main() -> Result<()> {
    let _session = setup_ml()?;
    let mut card = CardCapture::new(read_card_index()?, 512, SCREEN_SIZE)?;

    highgui::named_window(WINDOW_NAME, highgui::WINDOW_NORMAL)?;
    highgui::set_window_property(WINDOW_NAME, highgui::WND_PROP_TOPMOST, 1.0)?;
    highgui::resize_window(WINDOW_NAME, 420, 420)?;

    loop {
        highgui::imshow(WINDOW_NAME, &card.grab()?)?;
    }
}