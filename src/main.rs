use linear_algebra::matrix::{main_print, Matrix};
fn main() {
    println!("Hello, world!");
    let m1: Matrix = Matrix::rand(4, 4);
    let m2: Matrix = Matrix::ones(4, 4);
    let m3: Matrix = Matrix::upper_triangular(4, 4);
    let m4: Matrix = Matrix::lower_triangular(4, 4);
    let m5: Matrix = m3.mult(&m4);
    main_print(m1, m2, m3, m4, m5)
}
