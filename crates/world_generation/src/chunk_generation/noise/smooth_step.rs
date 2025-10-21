use std::f64::consts::PI;

use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct SmoothStep<T> {
    noise: T,
    steps: f64,
    smoothness: f64,
}

impl<T> SmoothStep<T> {
    pub const DEFAULT_STEPS: f64 = 1.;
    pub const DEFAULT_SMOOTHNESS: f64 = 0.25;

    pub fn new(source: T) -> Self {
        Self {
            noise: source,
            steps: Self::DEFAULT_STEPS,
            smoothness: Self::DEFAULT_SMOOTHNESS,
        }
    }

    pub fn set_steps(self, steps: f64) -> Self {
        Self { steps, ..self }
    }

    pub fn set_smoothness(self, smoothness: f64) -> Self {
        Self { smoothness, ..self }
    }
}

impl<T> Default for SmoothStep<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            noise: Default::default(),
            steps: Self::DEFAULT_STEPS,
            smoothness: Self::DEFAULT_SMOOTHNESS,
        }
    }
}

impl<T, TInput> NoiseFunction<NoiseResult, TInput> for SmoothStep<T>
where
    T: NoiseFunction<NoiseResult, TInput>,
{
    fn get(&self, _input: TInput) -> NoiseResult {
        // NOOP
        NoiseResult {
            value: 0.,
            derivative: [0., 0.],
        }
    }
}

// https://www.desmos.com/calculator/zyrixan1eo
fn smooth_floor(x: f64, factor: f64) -> f64 {
    let sigmoid_value = sigmoid((PI * x).sin(), factor);
    x + (2.0 * sigmoid_value - 1.0) * ((PI * x).cos().asin() / PI) - 0.5
}

fn sigmoid(x: f64, factor: f64) -> f64 {
    x / (factor + x.abs()) * 0.5 + 0.5
}
