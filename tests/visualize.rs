extern crate image;

const IMAGE_PATH: &str = "black_and_white_image.png";
const DATA_PATH: &str = "data/mnist/x_train.json";

#[test]
fn visualize() {
    miniai::visualization::black_and_white::black_and_white();
}
