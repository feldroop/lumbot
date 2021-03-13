use structopt::StructOpt;

use tokio::time::{sleep, Duration};
use tokio::{io::AsyncReadExt, fs::File};

use fantoccini::{ClientBuilder, Locator};

use winapi::um::{wingdi::GetPixel, winuser::GetDC};
use std::ptr::null_mut;

// Coordinates of the right branches (hardcoded for screen and wedriver)
const BRANCH_X: i32 = 369;
const BRANCH_YS: [i32; 6] = [685, 585, 485, 385, 285, 185];

// color value of a pixel inside a branch
const BROWN: u32 = 3699873;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The webdriver address, default is the local chromedriver on default port
    #[structopt(long = "driver", short = "d", default_value = "http://localhost:9515")]
    driver: String,
    /// the delay after a click
    #[structopt(long = "milliseconds-delay", short = "m", default_value = "250")]
    delay: u64,
    /// The path to a file with the url to the website of the desired lumberjack game
    #[structopt(long = "url-file", short = "u")]
    url_file: String,
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let args = Cli::from_args();

    // establish connection to webdriver
    let mut c = ClientBuilder::rustls()
        .connect(&args.driver)
        .await
        .expect("failed to connect to WebDriver");
    
    let mut f = File::open(&args.url_file)
        .await
        .expect("The file with the url could not be opened.");
    let mut url = String::new();

    f.read_to_string(&mut url)
        .await
        .expect("The file with the url could not be read.");

    // setup game
    c.goto(&url.trim()).await?;
    c.set_window_size(600, 1100).await?;
    c.set_window_position(0, 0).await?;
    c.find(Locator::Css(".button")).await?.click().await?;
    
    // find important elements
    let right = c.find(Locator::Css("#button_right.button")).await?;
    let left = c.find(Locator::Css("#button_left.button")).await?;
    
    // sleep to give the game some time for start-up
    sleep(Duration::from_millis(1000)).await;

    // get handle to the current screen
    let hdc = unsafe {
        GetDC(null_mut())
    };

    loop {
        // get colors at pixel positions of right branches
        let colors: Vec<u32> = BRANCH_YS
            .iter()
            .map(|y| unsafe { GetPixel(hdc, BRANCH_X, *y)})
            .collect();

        // check if color is brown -> there is a branch on the right
        for color in colors {
            if color == BROWN {
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
