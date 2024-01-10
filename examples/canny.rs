#![feature(test)]
extern crate test;

use image::GrayImage;
use rust_for_multimedia_2023::{matrix::Matrix};

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
        grayscale_pixels[i] = ((pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32) / 3) as u8;
    }

    save_grayscale("output/gray.png", grayscale_pixels.clone(), width, height);
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}
