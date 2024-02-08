extern crate image;

use candle_core::Tensor;
use candle_datasets::vision::mnist;
use std::error::Error;

use image::{ImageBuffer, Luma};
const IMAGE_PATH: &str = "black_and_white_image.png";
const DATA_PATH: &str = "data/mnist/x_train_ten.json";

fn sqrt_if_integer(number: usize) -> Result<usize, String> {
    let sqrt = (number as f64).sqrt();

    // Check if the square root is an integer by comparing it to its rounded value
    if sqrt.fract() == 0.0 {
        Ok(sqrt as usize)
    } else {
        Err("The square root is not an integer.".to_string())
    }
}

pub fn convert_one_dim_tensor_to_square_black_and_white_jpg(
    image: Tensor,
) -> Result<(), Box<dyn Error>> {
    // Example: Creating a 100x100 black and white image
    let len = image.dims()[0];
    let dims = sqrt_if_integer(len)?;
    println!("{:?}", dims);

    let two_d_tensor = image.reshape((dims, dims))?.to_vec2()?;
    println!("{:?}", two_d_tensor);

    // Initialize the image buffer
    let mut imgbuf = ImageBuffer::<Luma<u8>, Vec<u8>>::new(dims as u32, dims as u32);

    // Populate the image
    // Here, you would replace this loop with your own logic to set pixels based on your array's values
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Convert `x` and `y` to `usize` for indexing `two_d`
        let intensity: f32 = two_d_tensor[y as usize][x as usize];

        // Assuming `intensity` is normalized between 0.0 and 1.0, scale it to 0-255 range
        // Adjust this logic if your data range is different
        let scaled_intensity = (intensity.clamp(0.0, 1.0) * 255.0) as u8;

        *pixel = Luma([scaled_intensity]);
    }

    // Save the image
    imgbuf.save(IMAGE_PATH).unwrap();
    Ok(())
}
