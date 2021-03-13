use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;

use tokio::time::{sleep, Duration};
use fantoccini::{ClientBuilder, Locator};

use winapi::um::{wingdi::GetPixel, winuser::GetDC};
use std::ptr::null_mut;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The webdriver address, default is the local chromedriver on default port
    #[structopt(long = "driver", short = "d", default_value = "http://localhost:9515")]
    driver: String,
    /// the delay after a click
    #[structopt(long = "milliseconds-delay", short = "m", default_value = "250")]
    delay: u64,
    /// The path to a file with the pixel values for your lumberjack game screen/webdriver setup
    #[structopt(long = "pixel-file", short = "p")]
    pixel_file: String,
    /// The path to a file with the url to the website of the desired lumberjack game
    #[structopt(long = "url-file", short = "u")]
    url_file: String,
}

#[derive(Deserialize)]
struct PixelValues {
    /// x coordinate of a pixel inside the right branches of the the tree (regarding the whole screen)
    right_branch_x: i32,
    /// y coordinates of pixels inside each of the six branches of the tree (regarding the whole screen)
    branch_ys: [i32; 6],
    /// the RGBA value of a pixel inside a branch as a 32 bit integer
    brown: u32
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let args = Cli::from_args();

    let pixel_file = std::fs::File::open(&args.pixel_file)
        .expect("The file with the pixel values could not be opened.");
    let px: PixelValues = serde_json::from_reader(pixel_file)
        .expect("Failed to parse to pixel value file");

    let mut url_file = File::open(&args.url_file)
        .expect("The file with the url could not be opened.");
    let mut url = String::new();

    url_file.read_to_string(&mut url)
        .expect("The file with the url could not be read.");

    let mut c = ClientBuilder::rustls()
        .connect(&args.driver)
        .await
        .expect("failed to connect to WebDriver");

    // setup game
    c.goto(&url.trim()).await?;
    c.set_window_size(600, 1100).await?;
    c.set_window_position(0, 0).await?;
    c.find(Locator::Css(".button")).await?.click().await?;
    
    let right = c.find(Locator::Css("#button_right.button")).await?;
    let left = c.find(Locator::Css("#button_left.button")).await?;
    
    // sleep to give the game some time for start-up
    sleep(Duration::from_millis(1000)).await;

    // get handle to the current screen from windows
    let hdc = unsafe { 
        GetDC(null_mut())
    };

    loop {
        // get colors at pixel positions of right branches
        let colors: Vec<u32> = px.branch_ys
            .iter()
            .map(|y| unsafe { GetPixel(hdc, px.right_branch_x, *y)})
            .collect();

        // check if color is brown -> there is a branch on the right
        for color in colors {
            if color == px.brown {
                left.clone().click().await?;
                left.clone().click().await?;
            } else {
                right.clone().click().await?;
                right.clone().click().await?;
            }
        }
        // sleep to wait for the animation to complete
        sleep(Duration::from_millis(args.delay)).await;
    }
}
