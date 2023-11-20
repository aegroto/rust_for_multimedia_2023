use crate::matrix::Matrix;

#[cfg(test)]
mod tests;

pub fn normalize(x: u8) -> f32 {
    (x as f32) / 255.0
}

pub fn denormalize(v: f32) -> u8 {
    f32::round(v * 255.0) as u8
}

pub fn normalize_image_matrix(image: &Matrix<u8>) -> Matrix<f32> {
    Matrix::new(
        image.values().iter().map(|v| normalize(*v)).collect::<Vec<f32>>(), 
        image.width(), 
        image.height()
    )
}


pub fn denormalize_image_matrix(image: &Matrix<f32>) -> Matrix<u8> {
    Matrix::new(
        image.values().iter().map(|v| denormalize(*v)).collect::<Vec<u8>>(),
        image.width(), 
        image.height()
    )
}
