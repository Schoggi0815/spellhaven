use std::ops;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NoiseResult {
    pub value: f64,
    pub derivative: [f64; 2],
}

impl NoiseResult {
    pub fn new_constant(constant_value: f64) -> Self {
        Self {
            value: constant_value,
            derivative: [0., 0.],
        }
    }
}

impl ops::Add<NoiseResult> for NoiseResult {
    type Output = NoiseResult;

    #[inline]
    fn add(self, rhs: NoiseResult) -> Self::Output {
        NoiseResult {
            value: self.value + rhs.value,
            derivative: [
                self.derivative[0] + rhs.derivative[0],
                self.derivative[1] + rhs.derivative[1],
            ],
        }
    }
}

impl ops::Mul<NoiseResult> for NoiseResult {
    type Output = NoiseResult;

    #[inline]
    fn mul(self, rhs: NoiseResult) -> Self::Output {
        let value = self.value * rhs.value;

        let derivative_x =
            (self.value * rhs.derivative[0]) + (self.derivative[0] * rhs.value);
        let derivative_y =
            (self.value * rhs.derivative[1]) + (self.derivative[1] * rhs.value);

        NoiseResult {
            value,
            derivative: [derivative_x, derivative_y],
        }
    }
}

impl ops::Add<f64> for NoiseResult {
    type Output = NoiseResult;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        NoiseResult {
            value: self.value + rhs,
            derivative: self.derivative,
        }
    }
}

impl ops::Mul<f64> for NoiseResult {
    type Output = NoiseResult;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        NoiseResult {
            value: self.value * rhs,
            derivative: [self.derivative[0] * rhs, self.derivative[1] * rhs],
        }
    }
}

impl ops::Neg for NoiseResult {
    type Output = NoiseResult;

    #[inline]
    fn neg(self) -> Self::Output {
        NoiseResult {
            value: -self.value,
            derivative: [-self.derivative[0], -self.derivative[1]],
        }
    }
}
