extern crate image;

use image::{ImageBuffer, Luma};
const IMAGE_PATH: &str = "black_and_white_image.png";
const DATA_PATH: &str = "data/mnist/x_train_ten.json";

#[test]
fn visualize() {
    miniai::data_reading::read_json::read_json_to_vec_vec_f64(DATA_PATH);
}
