use core::panic;

use crate::{activation::Activation, matrix::Matrix};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: f64,
    activation: Activation<'a>,
}

impl Network<'_> {
    pub fn new<'a>(layers: Vec<usize>, activation: Activation<'a>, learning_rate: f64) -> Network {
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];
        for i in 0..layers.len() - 1 {
            weights.push(Matrix::rand(layers[i + 1], layers[i]));
            biases.push(Matrix::rand(layers[i + 1], 1));
        }
        println!(
            "{} {} {} :at network level",
            weights.len(),
            biases.len(),
            layers.len(),
        );
        Network {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate,
            activation,
        }
    }
    pub fn forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Invalid inputs length");
        }

        let mut current: Matrix = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            // if self.weights[i].cols != current.rows {
            //     println!(
            //         "Weights cols: {}, Current rows: {}",
            //         self.weights[i].cols, current.rows
            //     );

            //     panic!("Incompatible dimensions for matrix multiplication");
            // }
            current = self.weights[i]
                .mult(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            self.data.push(current.clone());
        }

        current.transpose().data[0].to_owned()
    }
    pub fn back_propagate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers[self.layers.len() - 1] {
            panic!("Invalid targets length");
        }

        let mut parsed = Matrix::from(vec![outputs]).transpose();
        let mut errors = Matrix::from(vec![targets]).transpose().sub(&parsed);
        let mut gradients = parsed.map(self.activation.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients.dot(&errors).map(&|x: f64| x * self.learning_rate);

            self.weights[i] = self.weights[i].add(&gradients.mult(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().mult(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }
    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                let mut total_error: f64 = 0.0;
                for j in 0..inputs.len() {
                    let outputs = self.forward(inputs[j].clone());
                    let target = &targets[j];
                    for k in 0..outputs.len() {
                        total_error += (outputs[k] - target[k]).powi(2);
                    }
                }
                println!(
                    "Epoch {}: error = {}",
                    i,
                    total_error / (inputs.len() as f64)
                );
                println!("0 and 0 :{:?}", self.forward(vec![0.0, 0.0]));
                println!("0 and 1 :{:?}", self.forward(vec![0.0, 1.0]));
                println!("1 and 0 :{:?}", self.forward(vec![1.0, 0.0]));
                println!("1 and 1 :{:?}", self.forward(vec![1.0, 1.0]));
            }
            for j in 0..inputs.len() {
                let outputs = self.forward(inputs[j].clone());
                self.back_propagate(outputs, targets[j].clone());
            }
        }
    }
}
