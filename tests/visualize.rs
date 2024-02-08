use std::error::Error;

use candle_datasets::vision::mnist;

extern crate image;

#[test]
pub fn visualize() -> Result<(), Box<dyn Error>> {
    let mnist = mnist::load()?;
    let image = mnist.test_images.get(1)?;
    let result =
        miniai::visualization::black_and_white::convert_one_dim_tensor_to_square_black_and_white_jpg(
            image,
        );

    Ok(())
}
