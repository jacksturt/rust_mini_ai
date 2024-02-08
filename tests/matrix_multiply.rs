use std::error::Error;

use candle_core::{DType, Device, IndexOp, Tensor};
use candle_datasets::vision::mnist;

extern crate image;

#[test]
pub fn matrix_multiply() -> Result<(), Box<dyn Error>> {
    let mnist = mnist::load()?;
    let mnist_train = mnist.train_images;
    let first_five = mnist_train.i(0..5)?;
    let first_five_vec = first_five.to_vec2::<f32>()?;
    let weights = Tensor::rand(-1.0f32, 1.0f32, (784, 10), &Device::Cpu)?;
    let weights_vec = weights.to_vec2::<f32>()?;
    println!("{:?}", first_five);
    println!("{:?}", weights);
    let result_tensor = first_five.matmul(&weights)?;
    let result_tensor_vec = result_tensor.to_vec2::<f32>()?;
    let result_vec = miniai::matrix_multiplication::matrix_multiply::matrix_multiply(
        &first_five_vec,
        &weights_vec,
    );
    let tolerance = 0.0001;
    for i in 0..result_tensor.dims()[0] {
        for j in 0..result_tensor.dims()[1] {
            let tensor_result = result_tensor_vec[i][j];
            let vec_result = result_vec[i][j];
            assert!((tensor_result - vec_result).abs() < tolerance);
        }
    }

    Ok(())
}
