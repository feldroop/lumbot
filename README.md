# lumbot
A bot for the Telegram HTML game lumberjack

## Usage

You need:

* The toolchain of the Rust programming language (cargo)
* A webdriver, e.g. chromedriver
* A text file with the url to your lumberjack game
* A JSON file with pixel values of your screen with the following Syntax:

  ```JSON
  {
      "right_branch_x": 123,
      "branch_ys": [700, 600, 500, 400, 300, 200],
      "brown": 3699873
  }
  ```

  * `right_branch_x`: x coordinate of a pixel inside the right branches of the the tree (regarding the whole screen)
  * `branch_ys`: y coordinates of pixels inside each of the six branches of the tree (regarding the whole screen)
  * `brown`: the RGBA value of a pixel inside a branch as a 32 bit integer

Then you can start your webdriver in a separate shell and run the following:

```
cargo run --release -- --pixel-file .\my_pixel_file.json --url-file .\my_url_file.txt
```
