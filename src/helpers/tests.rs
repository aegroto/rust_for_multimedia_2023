use std::f32::EPSILON;

use crate::helpers::denormalize;

use super::normalize;

#[test] 
fn normalize_zero() {
    let result = normalize(0);
    assert!(result - 0.0 < EPSILON);
}

#[test] 
fn denormalize_zero() {
    let result = denormalize(0.0);
    assert_eq!(result, 0);
}