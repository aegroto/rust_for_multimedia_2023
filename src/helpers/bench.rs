use test::Bencher;

use crate::matrix::Matrix;

fn load_matrix() -> Matrix<u8> {
    let image = image::io::Reader::open("assets/myownlena.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    let pixels = image.as_flat_samples().as_slice().to_vec();
    Matrix::new(
        pixels,
        image.width() as usize,
        image.height() as usize
    )
}

#[bench]
fn bench_iter(bencher: &mut Bencher) {
    let matrix = load_matrix();

    bencher.iter(|| super::normalize_image_matrix(&matrix));
}

#[bench]
fn bench_pariter(bencher: &mut Bencher) {
    let matrix = load_matrix();

    bencher.iter(|| super::normalize_image_matrix_pariter(&matrix));
}