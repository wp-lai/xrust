use clap::Parser;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    path: String,

    #[arg(long, default_value_t = String::from("image.png"))]
    output: String,

    #[arg(long, default_value_t = 1920)]
    width: u32,

    #[arg(long, default_value_t = 1080)]
    height: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let options = LaunchOptionsBuilder::default()
        .window_size(Some((args.width, args.height)))
        .build()?;

    let browser = Browser::new(options)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(&args.path)?;
    tab.wait_until_navigated()?;

    let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;

    fs::write(&args.output, png_data)?;

    Ok(())
}
