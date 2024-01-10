#![feature(test)]
extern crate test;

use std::f32::consts::E;

use image::{GrayImage, save_buffer};
use itertools::Itertools;
use rust_for_multimedia_2023::{
    conv::conv2d,
    drog,
    helpers::{denormalize_image_matrix, normalize_image_matrix},
    matrix::Matrix, edge::{Edge, ThresholdedEdge}, nonmax_suppression::perform_nonmax_suppression, hysteresis_thresholding::perform_hysteresis_thresholding,
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

    println!("Applying DroG...");
    let (x_drog_kernel, y_drog_kernel) = drog::kernel(7, 1.0);

    let x_drog_response = conv2d(&normalized_image, &x_drog_kernel);
    let y_drog_response = conv2d(&normalized_image, &y_drog_kernel);

    save_normalized_image("output/x_drog_response.png", x_drog_response.clone());
    save_normalized_image("output/y_drog_response.png", y_drog_response.clone());

    println!("Calculating edges...");
    let edges_matrix = drog_response_to_edge(&x_drog_response, &y_drog_response);
    save_edges_magnitude("output/edges_magnitude.png", edges_matrix.clone());

    println!("Suppressing edges...");
    let suppressed_edges_matrix = perform_nonmax_suppression(&edges_matrix, 5);
    save_edges_magnitude("output/suppressed_edges_magnitude.png", suppressed_edges_matrix.clone());

    println!("Thresholding edges...");
    let suppressed_edges_matrix = perform_nonmax_suppression(&edges_matrix, 5);
    let thresholded_edges = perform_hysteresis_thresholding(&suppressed_edges_matrix, 0.2, 0.5);
    save_thresholded_edges("output/thresholded_edges.png", thresholded_edges.clone());
}

fn drog_response_to_edge(x_matrix: &Matrix<f32>, y_matrix: &Matrix<f32>) -> Matrix<Edge> {
    let mut edges = Vec::new();

    let x_values = x_matrix.values();
    let y_values = y_matrix.values();

    for i in 0..x_values.len() {
        edges.push(Edge::new(x_values[i], y_values[i]));
    }

    Matrix::new(edges, x_matrix.width(), x_matrix.height())
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

fn save_edges_magnitude(path: &str, edges: Matrix<Edge>) {
    let magnitudes_image = Matrix::new(
        edges.values().iter().map(|edge| edge.get_magnitude()).collect_vec(),
        edges.width(),
        edges.height()
    );
    let denormalized = denormalize_image_matrix(&magnitudes_image);
    let plane = GrayImage::from_raw(
        denormalized.width() as u32,
        denormalized.height() as u32,
        denormalized.values().clone(),
    )
    .unwrap();
    plane.save(path).unwrap();
}

fn save_thresholded_edges(path: &str, edges: Matrix<ThresholdedEdge>) {
    let magnitudes_image = Matrix::new(
        edges.values().iter().map(|edge| match edge {
            ThresholdedEdge::STRONG => 1.0,
            ThresholdedEdge::WEAK => 0.5,
            ThresholdedEdge::NULL => 0.0,
        }).collect_vec(),
        edges.width(),
        edges.height()
    );
    let denormalized = denormalize_image_matrix(&magnitudes_image);
    let plane = GrayImage::from_raw(
        denormalized.width() as u32,
        denormalized.height() as u32,
        denormalized.values().clone(),
    )
    .unwrap();
    plane.save(path).unwrap();
}