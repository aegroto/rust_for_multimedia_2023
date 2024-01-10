#![feature(test)]
extern crate test;

use std::f32::consts::E;

use image::GrayImage;
use rust_for_multimedia_2023::{
    conv::conv2d,
    drog,
    helpers::{denormalize_image_matrix, normalize_image_matrix},
    matrix::Matrix,
};

fn main() {
    let image = image::io::Reader::open("assets/myownlena.png")
        .unwrap()
        .decode()
        .unwrap();

    let width = image.width();
    let height = image.height();
    let num_pixels = (width * height) as usize;

    let mut grayscale_pixels = vec![0u8; num_pixels];

    for (i, pixel) in image.as_rgb8().unwrap().pixels().enumerate() {
        grayscale_pixels[i] =
            ((pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32) / 3) as u8;
    }

    save_grayscale("output/gray.png", grayscale_pixels.clone(), width, height);

    let image_matrix = Matrix::new(grayscale_pixels, width as usize, height as usize);
    let normalized_image = normalize_image_matrix(&image_matrix);

    let (x_drog_kernel, y_drog_kernel) = drog::kernel(7, 1.0);

    let x_drog_response = conv2d(&normalized_image, &x_drog_kernel);
    let y_drog_response = conv2d(&normalized_image, &y_drog_kernel);

    save_normalized_image("output/x_drog_response.png", x_drog_response);
    save_normalized_image("output/y_drog_response.png", y_drog_response);
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}

fn save_normalized_image(path: &str, normalized_image: Matrix<f32>) {
    let denormalized = denormalize_image_matrix(&normalized_image);
    let plane = GrayImage::from_raw(
        denormalized.width() as u32,
        denormalized.height() as u32,
        denormalized.values().clone(),
    )
    .unwrap();
    plane.save(path).unwrap();
}
