use crate::{matrix::Matrix, helpers::{normalize, denormalize, normalize_image_matrix, denormalize_image_matrix}, conv::conv2d};

pub enum SobelMode {
    HORIZONTAL,
    VERTICAL
}

impl SobelMode {
    pub fn get_kernel(&self) -> Matrix<f32> {
        match self {
            SobelMode::HORIZONTAL => horizontal_kernel(),
            SobelMode::VERTICAL => vertical_kernel(),
        }
    }
}

pub fn sobel(image: Matrix<u8>, mode: SobelMode) -> Matrix<u8> {
    let normalized_image = normalize_image_matrix(&image);
    let kernel = mode.get_kernel();
    let grad_image = conv2d(&normalized_image, &kernel);
    let denormalized_image = denormalize_image_matrix(&grad_image);
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