use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    pub fn ones(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![1.0; cols]; rows],
        }
    }
    pub fn upper_triangular(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;
        for i in 0..rows {
            for j in i..cols {
                data[i][j] = 1.0;
            }
        }
        Matrix {
            rows,
            cols,
            data: data,
        }
    }
    pub fn lower_triangular(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;
        for i in 0..rows {
            for j in 0..=i {
                data[i][j] = 1.0;
            }
        }
        Matrix { rows, cols, data }
    }
    pub fn rand(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;

        for i in 0..rows {
            for j in 0..cols {
                data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        Matrix { rows, cols, data }
    }
    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        res
    }

    pub fn mult(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }

                res.data[i][j] = sum;
            }
        }

        res
    }
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }
    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            self.data
                .clone()
                .into_iter()
                .map(|row: Vec<f64>| row.into_iter().map(|value: f64| function(value)).collect())
                .collect(),
        )
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn sub(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.rows && self.cols == other.cols);
        let mut data: Vec<Vec<f64>> = Matrix::zeros(self.rows, self.cols).data;
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: data,
        }
    }
    pub fn dot(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        res
    }
    pub fn transpose(&self) -> Matrix {
        let mut data: Vec<Vec<f64>> = Matrix::zeros(self.cols, self.rows).data;
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: data,
        }
    }
}
