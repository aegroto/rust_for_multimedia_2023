use crate::matrix::Matrix;

pub fn kernel(size: usize, std_dev: f32) -> (Matrix<f32>, Matrix<f32>) {
    let std_dev_square = std_dev.powi(2);

    let mut x_kernel = Matrix::new(vec![0.0; size * size], size, size);
    let mut y_kernel = Matrix::new(vec![0.0; size * size], size, size);

    for x in 0..size {
        for y in 0..size {
            let x_f = (x as i32 - size as i32 / 2) as f32;
            let y_f = (y as i32 - size as i32 / 2) as f32;

            let x_square = (x_f * x_f) as f32;
            let y_square = (y_f * y_f) as f32;

            let exp_coefficient = -(x_square + y_square) / (2.0 * std_dev_square);

            x_kernel.set(x, y, -(x_f / std_dev_square) * f32::exp(exp_coefficient));
            y_kernel.set(x, y, -(y_f / std_dev_square) * f32::exp(exp_coefficient));
        }
    }

    (x_kernel, y_kernel)
}