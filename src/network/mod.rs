use core::panic;

use crate::{
    activation::{self, Activation},
    matrix::Matrix,
};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation<'a>,
}

impl Network<'_> {
    pub fn new<'a>(layers: Vec<usize>, activation: Activation<'a>) -> Network {
        let mut weights: Vec<Matrix> = Vec::new();
        let mut biases: Vec<Matrix> = Vec::new();
        let mut data: Vec<Matrix> = Vec::new();
        for i in 0..layers.len() - 1 {
            weights.push(Matrix::rand(layers[i], layers[i + 1]));
            biases.push(Matrix::zeros(1, layers[i + 1]));
            data.push(Matrix::zeros(1, layers[i + 1]));
        }
        Network {
            layers,
            weights,
            biases,
            data,
            activation,
        }
    }
    pub fn forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Input must have the same number of columns as the first layer");
        }
        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
                .mult(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            self.data.push(current.clone());
        }
        current.data[0].to_owned()
    }
    pub fn back_propagate(&mut self, target: Vec<f64>, learning_rate: f64) {
        let output_error = Matrix::from(vec![target]).transpose() - self.data[self.data.len() - 1];
        self.data[self.data.len() - 1] = Matrix::zeros(1, self.data[self.data.len() - 1].len());

        for i in (0..self.data.len() - 1).rev() {
            self.data[i] = self.data[i]
                .map(self.activation.derivative)
                .mult(&output_error)
                .mult(&self.weights[i].transpose());
            output_error = self.data[i].mult(&self.weights[i]);
        }
        for i in 0..self.data.len() - 1 {
            self.weights[i] = self.weights[i]
                .add(&self.data[i].transpose().mult(&self.data[i + 1]))
                .mult(learning_rate);
            self.biases[i] = self.biases[i].add(&self.data[i].transpose().mult(&output_error));
        }

        self.data[0] = Matrix::zeros(1, self.data[0].len());

        for i in 0..self.data.len() - 1 {
            self.data[i] = self.data[i].add(&self.data[i + 1].mult(&self.weights[i]));
        }

        self.data[self.data.len() - 1] = Matrix::zeros(1, self.data[self.data.len() - 1].len());
    }
}
