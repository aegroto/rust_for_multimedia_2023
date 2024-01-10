use crate::{
    edge::{Edge, ThresholdedEdge},
    matrix::Matrix,
    nonmax_suppression::neighbour_at,
};

pub fn perform_edge_linking(
    edges: &Matrix<Edge>,
    thresholded_edges: &Matrix<ThresholdedEdge>,
    distance_range: usize
) -> Matrix<ThresholdedEdge> {
    let mut linked_edges = thresholded_edges.clone();

    for x in 0..edges.width() {
        for y in 0..edges.height() {
            if has_strong_neighbour(edges, thresholded_edges, x, y, distance_range) {
                linked_edges.set(x, y, ThresholdedEdge::STRONG)
            } else {
                linked_edges.set(x, y, ThresholdedEdge::NULL)
            }
        }
    }

    linked_edges
}

pub fn has_strong_neighbour(
    edges: &Matrix<Edge>,
    thresholded_edges: &Matrix<ThresholdedEdge>,
    x: usize,
    y: usize,
    distance_range: usize,
) -> bool {
    let distance_range = distance_range as i32;
    let thresholded_edge = thresholded_edges.get(x, y);

    if matches!(thresholded_edge, ThresholdedEdge::STRONG) {
        return true;
    }

    let edge = edges.get(x, y);
    let (x_dir, y_dir) = edge.dir_norm();
    let x_tan_dir = -y_dir;
    let y_tan_dir = x_dir;

    for distance in -distance_range..distance_range {
        match neighbour_at(thresholded_edges, x_tan_dir, y_tan_dir, x, y, distance) {
            Some(neighbour) => {
                if matches!(neighbour, ThresholdedEdge::STRONG) {
                    return true;
                }
            }
            None => continue,
        }
    }

    false
}
