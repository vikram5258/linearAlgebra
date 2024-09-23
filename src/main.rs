use std::vec;

use linear_algebra::{activation::SIGMOID, network::Network};
fn main() {
    let mut network = Network::new(vec![2, 3, 1], SIGMOID, 0.5);
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let outputs: Vec<Vec<f64>> = vec![vec![1.0], vec![0.0], vec![0.0], vec![1.0]];
    network.train(inputs, outputs, 10000);
}
