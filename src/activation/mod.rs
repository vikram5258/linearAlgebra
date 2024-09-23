#[derive(Clone)]
pub struct Activation<'a> {
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}
pub const SIGMOID: Activation = Activation {
    function: &|x: f64| 1.0 / (1.0 + (-x).exp()),
    derivative: &|x: f64| x * (1.0 - x),
};
pub const TANH: Activation = Activation {
    function: &|x| (2.0 / (1.0 + (-x).exp())).sqrt() - 1.0,
    derivative: &|x| 1.0 - (x * x),
};
pub const RELU: Activation = Activation {
    function: &|x| if x > 0.0 { x } else { 0.0 },
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.0 },
};
pub const LEAKY_RELU: Activation = Activation {
    function: &|x| if x > 0.0 { x } else { 0.01 * x },
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.01 }, // 0.01
};
pub const ELU: Activation = Activation {
    function: &|x| if x > 0.0 { x } else { 0.01 * (x.exp() - 1.0) },
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.01 * (1.0 - x) }, // 0.01
};
pub const SELU: Activation = Activation {
    function: &|x| if x > 0.0 { x } else { 0.01 * (x.exp() - 1.0) },
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.01 * (1.0 - x) }, // 0.01
};
