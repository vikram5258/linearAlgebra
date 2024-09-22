use rand::{thread_rng, Rng};
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows:usize, cols:usize) -> Matrix {
        Matrix{ 
            rows,
            cols,
            data: vec![vec![0.0; cols] ;rows],
         }
    }
    pub fn ones(rows:usize, cols:usize) -> Matrix {
        Matrix{ 
            rows,
            cols,
            data: vec![vec![1.0; cols] ;rows],
         }
    }
    pub fn upper_triangular(rows:usize, cols:usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;
        for i in 0..rows {
            for j in i..cols {
                data[i][j] = 1.0;
            }
        }
        Matrix{ 
            rows,
            cols,
            data: data,
         }
    }
    pub fn lower_triangular(rows:usize, cols:usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;
        for i in 0..rows {
            for j in 0..=i {
                data[i][j] = 1.0;            }
        }
        Matrix{ 
            rows,
            cols,
            data
         }
    }
    pub fn rand(rows:usize, cols:usize) -> Matrix {
        let mut rng = thread_rng();
        let mut data: Vec<Vec<f64>> = Matrix::zeros(rows, cols).data;

        for i in 0..rows {
            for j in 0..cols {
                data[i][j] = rng.gen::<f64>() *2.0 - 1.0;
            }
        }
        Matrix{ 
            rows,
            cols,
            data,
         }
    }
    pub fn mult(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);
        let mut data: Vec<Vec<f64>> = Matrix::zeros(self.rows, other.cols).data;

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix{ 
            rows: self.rows,
            cols: other.cols,
            data: data,
         }
    }
}

pub fn main_print(m1: Matrix, m2: Matrix, m3: Matrix, m4: Matrix) {
    println!("Matrix 1:\n{:?}", m1.data);
    println!("Matrix 2:\n{:?}", m2.data);
    println!("Matrix 3:\n{:?}", m3.data);
    println!("Matrix 4:\n{:?}", m4.data);
}

