# Linear Algebra and Neural Network Library
=============================================

This library provides a basic implementation of linear algebra and neural network functionality in Rust. It allows you to perform matrix operations, create and train neural networks, and use various activation functions.

## Features

### Linear Algebra Operations

The `Matrix` struct provides basic linear algebra operations such as matrix multiplication and random initialization. Here's an example of creating a matrix and performing matrix multiplication:

```rust
let matrix_a = Matrix::rand(2, 3);
let matrix_b = Matrix::rand(3, 2);
let matrix_c = matrix_a.dot(&matrix_b);
let network = Network::new(vec![2, 3, 1],Activation::new(), 0.1);
network.train(vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0]], vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]], 100);


let activation = Activation::new_sigmoid();
let output = activation.apply(&vec![0.5]);


[dependencies]
linear_algebra = "0.1.0"
```

### Linear Algebra Operations
```rust
extern crate linear_algebra;

let matrix_a = Matrix::rand(2, 3);
let matrix_b = Matrix::rand(3, 2);
let matrix_c = matrix_a.dot(&matrix_b);
let network = Network::new(vec![2, 3, 1], Activation::new(), 0.1);
network.train(vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0]], vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]], 100);
let activation = Activation::new_sigmoid();
let output = activation.apply(&vec![0.5]);
```
I hope this helps! Let me know if you have any further questions.