use crate::{matrix::Matrix, helpers::{normalize, denormalize}, conv::conv2d};

pub enum SobelMode {
    HORIZONTAL,
    VERTICAL
}

pub fn sobel(image: Matrix<u8>, mode: SobelMode) -> Matrix<u8> {
    let normalized_pixels = image.values().iter().map(|v| normalize(*v)).collect::<Vec<f32>>();
    let normalized_image = Matrix::new(normalized_pixels, image.width(), image.height());

    let kernel = match mode {
        SobelMode::HORIZONTAL => horizontal_kernel(),
        SobelMode::VERTICAL => vertical_kernel(),
    };

    let grad_image = conv2d(&normalized_image, &kernel);

    let denormalized_pixels = grad_image.values().iter().map(|v| denormalize(*v)).collect::<Vec<u8>>();
    let denormalized_image = Matrix::new(denormalized_pixels, image.width(), image.height());
    denormalized_image
}

fn horizontal_kernel() -> Matrix<f32> {
    Matrix::new([
        0.0, 0.0, 0.0,
        -1.0, 0.0, 1.0,
        0.0, 0.0, 0.0
    ].to_vec(), 3, 3)
}

fn vertical_kernel() -> Matrix<f32> {
    Matrix::new([
        0.0, -1.0, 0.0,
        0.0, 0.0, 0.0,
        0.0, 1.0, 0.0
    ].to_vec(), 3, 3)
}