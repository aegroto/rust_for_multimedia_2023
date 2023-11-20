use std::fmt::{Display, Debug};

pub struct Matrix<T: Copy> {
    values: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn new(values: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            values,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn values(&self) -> &Vec<T> {
        &self.values
    }

    #[allow(dead_code)]
    pub fn values_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }

    fn check_indices(&self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.check_indices(x, y);
        self.values[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.check_indices(x, y);
        self.values[y * self.width + x] = value
    }

    pub fn flipped(&self) -> Self {
        let mut flipped_matrix = Self {
            values: self.values.clone(),
            width: self.width,
            height: self.height
        };

        for x in 0..self.width {
            for y in 0..self.height {
                flipped_matrix.set(x, y, self.get(self.width - 1 - x, self.height - 1 - y));
            }
        }

        flipped_matrix 
    }

    #[allow(dead_code)]
    pub fn transposed(&self) -> Self {
        let mut transposed = Self {
            values: self.values.clone(),
            width: self.height,
            height: self.width
        };

        for x in 0..self.width {
            for y in 0..self.height {
                transposed.set(x, y, self.get(y, x));
            }
        }

        transposed 
    }
}

impl<T: Copy> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T: Copy + Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Ok(());

        for y in 0..self.width() {
            for x in 0..self.height() {
                result = f.write_str(&format!("{} ", self.get(x, y)));
                result.unwrap();
            }
            result = f.write_str("\n");
            result.unwrap();
        }

        result
    }
}