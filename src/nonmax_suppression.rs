use crate::{edge::Edge, matrix::Matrix};

pub fn perform_nonmax_suppression(edges: &Matrix<Edge>, distance_range: usize) -> Matrix<Edge> {
    let mut suppressed_edges = edges.clone();
    for x in 0..edges.width() {
        for y in 0..edges.height() {
            if !is_local_max(edges, x, y, distance_range) {
                suppressed_edges.set(x, y, Edge::zero());
            }
        }
    }

    suppressed_edges
}

pub fn neighbour_at<E: Copy>(
    edges: &Matrix<E>,
    x_dir: f32,
    y_dir: f32,
    x: usize,
    y: usize,
    distance: i32,
) -> Option<E> {
    let neighbour_x_offset =
        (x_dir.signum() as i32) * (if x_dir.abs() > 0.25 { distance } else { 0 });
    let neighbour_y_offset =
        (y_dir.signum() as i32) * (if y_dir.abs() > 0.25 { distance } else { 0 });

    let neighbour_x = x as i32 + neighbour_x_offset;
    let neighbour_y = y as i32 + neighbour_y_offset;

    if neighbour_x < 0
        || neighbour_x >= edges.width() as i32
        || neighbour_y < 0
        || neighbour_y >= edges.height() as i32
    {
        None
    } else {
        Some(edges.get(neighbour_x as usize, neighbour_y as usize))
    }
}

pub fn is_local_max(edges: &Matrix<Edge>, x: usize, y: usize, distance_range: usize) -> bool {
    let distance_range = distance_range as i32;
    let edge = edges.get(x, y);
    let (x_dir, y_dir) = edge.dir_norm();

    for distance in -distance_range..distance_range {
        match neighbour_at(edges, x_dir, y_dir, x, y, distance) {
            Some(neighbour) => {
                if edge.get_magnitude() < neighbour.get_magnitude() {
                    return false;
                }
            }
            None => continue,
        }
    }

    true
}
