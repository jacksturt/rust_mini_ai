extern crate image;

use std::error::Error;

use image::{ImageBuffer, Luma};
const IMAGE_PATH: &str = "black_and_white_image.png";
const DATA_PATH: &str = "data/mnist/x_train_single.json";

pub fn black_and_white() -> Result<(), Box<dyn Error>> {
    // Example: Creating a 100x100 black and white image
    let width = 28;
    let height = 28;
    let image_data: Vec<Vec<f64>> =
        crate::data_reading::read_json::read_json_to_vec_vec_f64(DATA_PATH)?;

    let three_d: Vec<Vec<Vec<f64>>> = image_data
        .into_iter()
        .map(|x| {
            let mut two_d: Vec<Vec<f64>> = Vec::with_capacity(28);
            for col in 0..28 {
                let mut row_vec: Vec<f64> = Vec::with_capacity(28);
                for row in 0..28 {
                    // Reverse the vertical order by adjusting the row calculation
                    row_vec.push(x[row * 28 + col]); // Adjusted index calculation
                }
                // Push rows from bottom to top to invert the vertical order
                two_d.insert(0, row_vec); // This inverts the vertical ordering
            }
            two_d.reverse();
            two_d
        })
        .collect();

    // Initialize the image buffer
    let mut imgbuf = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width, height);

    // Populate the image
    // Here, you would replace this loop with your own logic to set pixels based on your array's values
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Convert `x` and `y` to `usize` for indexing `two_d`
        let intensity = three_d[0][x as usize][y as usize];

        // Assuming `intensity` is normalized between 0.0 and 1.0, scale it to 0-255 range
        // Adjust this logic if your data range is different
        let scaled_intensity = (intensity.clamp(0.0, 1.0) * 255.0) as u8;

        *pixel = Luma([scaled_intensity]);
    }

    // Save the image
    imgbuf.save(IMAGE_PATH).unwrap();
    Ok(())
}
