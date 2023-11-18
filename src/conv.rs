use crate::matrix::Matrix;

pub fn conv2d(image: &Matrix<f32>, kernel: &Matrix<f32>) -> Matrix<f32>{
    let mut convolved_image = Matrix::new(vec![0f32; image.values().len()], image.width(), image.height());

    let flipped_kernel = kernel.flipped();

    let half_width = flipped_kernel.width() as i32 / 2;
    let half_height = flipped_kernel.height() as i32 / 2;

    for ix in 0..image.width() {
        for iy in 0..image.height() {
            let mut convolved_value = 0.0f32;

            for kx in 0..flipped_kernel.width() {
                for ky in 0..flipped_kernel.height() {
                    let k = flipped_kernel.get(kx, ky);

                    let px = ix as i32 + kx as i32 - half_width;
                    let py = iy as i32 + ky as i32 - half_height;

                    if px < 0 || px >= image.width() as i32 ||
                        py < 0 || py >= image.height() as i32 {
                            continue;
                        }
                    let p = image.get(px as usize, py as usize);

                    convolved_value += k * p;
                }
            }

            convolved_image.set(ix, iy, convolved_value);
        }
    }

    convolved_image
}