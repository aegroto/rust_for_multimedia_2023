use itertools::Itertools;

use crate::{
    edge::{Edge, ThresholdedEdge},
    matrix::Matrix,
};

pub fn perform_hysteresis_thresholding(
    edges: &Matrix<Edge>,
    weak_threshold: f32,
    strong_threshold: f32,
) -> Matrix<ThresholdedEdge> {
    Matrix::new(
        edges
            .values()
            .iter()
            .map(|edge| {
                if edge.get_magnitude() < weak_threshold {
                    ThresholdedEdge::NULL
                } else if edge.get_magnitude() < strong_threshold {
                    ThresholdedEdge::WEAK
                } else {
                    ThresholdedEdge::STRONG
                }
            })
            .collect_vec(),
        edges.width(),
        edges.height(),
    )
}
