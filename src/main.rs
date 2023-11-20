#![feature(test)]
extern crate test;

use image::GrayImage;
use matrix::Matrix;
use sobel::SobelMode;

mod matrix;
mod conv;
mod helpers;
mod sobel;

fn main() {
    let image = image::io::Reader::open("assets/myownlena.png")
        .unwrap()
        .decode()
        .unwrap();

    let width = image.width();
    let height = image.height();
    let num_pixels = (width * height) as usize;

    let mut r_plane = vec![0u8; num_pixels];
    let mut g_plane = vec![0u8; num_pixels];
    let mut b_plane = vec![0u8; num_pixels];

    for (i, pixel) in image.as_rgb8().unwrap().pixels().enumerate() {
        r_plane[i] = pixel.0[0];
        g_plane[i] = pixel.0[1];
        b_plane[i] = pixel.0[2];
    }

    save_grayscale("output/r_plane.png", r_plane.clone(), width, height);
    save_grayscale("output/g_plane.png", g_plane.clone(), width, height);
    save_grayscale("output/b_plane.png", b_plane.clone(), width, height);

    save_grads(r_plane.clone(), "output/r_hgrads.png", width, height, SobelMode::HORIZONTAL);
    save_grads(g_plane.clone(), "output/g_hgrads.png", width, height, SobelMode::HORIZONTAL);
    save_grads(b_plane.clone(), "output/b_hgrads.png", width, height, SobelMode::HORIZONTAL);

    save_grads(r_plane, "output/r_vgrads.png", width, height, SobelMode::VERTICAL);
    save_grads(g_plane, "output/g_vgrads.png", width, height, SobelMode::VERTICAL);
    save_grads(b_plane, "output/b_vgrads.png", width, height, SobelMode::VERTICAL);
}

fn save_grads(plane: Vec<u8>, path: &str, width: u32, height: u32, mode: SobelMode) {
    let matrix = Matrix::new(plane, width as usize, height as usize);
    let grads = sobel::sobel(matrix, mode);
    save_grayscale(path, grads.values().clone(), width, height);
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}
