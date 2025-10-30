use std::f64::consts::E;

use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct GFT<T> {
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
    pub gradient: f64,
    pub amplitude: f64,

    source: T,
    scale_factor: f64,
}

pub const DEFAULT_OCTAVE_COUNT: usize = 6;
pub const DEFAULT_FREQUENCY: f64 = 1.0;
pub const DEFAULT_LACUNARITY: f64 = 2.0;
//pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;
pub const DEFAULT_GRADIENT: f64 = 1.;
pub const DEFAULT_AMPLITUDE: f64 = 1.;

impl<T> GFT<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    pub fn new_with_source(source: T) -> Self {
        Self {
            octaves: DEFAULT_OCTAVE_COUNT,
            frequency: DEFAULT_FREQUENCY,
            lacunarity: DEFAULT_LACUNARITY,
            persistence: DEFAULT_PERSISTENCE,
            gradient: DEFAULT_GRADIENT,
            amplitude: DEFAULT_AMPLITUDE,
            source: source,
            scale_factor: Self::calc_scale_factor(
                DEFAULT_PERSISTENCE,
                DEFAULT_OCTAVE_COUNT,
            ),
        }
    }
}

impl<T> GFT<T> {
    pub fn set_amplitude(self, amplitude: f64) -> Self {
        Self { amplitude, ..self }
    }

    pub fn set_gradient(self, gradient: f64) -> Self {
        Self { gradient, ..self }
    }

    fn get_gradient_influence(&self, flatness: f64) -> f64 {
        //1. / (1. + (flatness * self.gradient))
        (E * 0.375).powf(-(flatness * self.gradient).powi(2))
    }

    pub fn set_octaves(self, octaves: usize) -> Self {
        if self.octaves == octaves {
            return self;
        }

        Self {
            octaves,
            scale_factor: Self::calc_scale_factor(self.persistence, octaves),
            ..self
        }
    }

    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    pub fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    pub fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            scale_factor: Self::calc_scale_factor(persistence, self.octaves),
            ..self
        }
    }

    fn calc_scale_factor(persistence: f64, octaves: usize) -> f64 {
        let denom =
            (1..=octaves).fold(0.0, |acc, x| acc + persistence.powi(x as i32));

        1.0 / denom
    }
}

/// 2-dimensional Fbm noise
impl<T> NoiseFunction<NoiseResult, [f64; 2]> for GFT<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    fn get(&self, point: [f64; 2]) -> NoiseResult {
        let mut result = NoiseResult::new_constant(0.);
        let mut total_flatness = 0.;

        for x in 0..self.octaves as i32 {
            let frequency = self.frequency * self.lacunarity.powi(x);
            let amplitude = self.amplitude * self.persistence.powi(x);

            // Get the signal.
            let mut noise_value = self
                .source
                .get([point[0] * frequency, point[1] * frequency]);

            noise_value.derivative = [
                noise_value.derivative[0] * frequency,
                noise_value.derivative[1] * frequency,
            ];

            let magnitude = (noise_value.derivative[0].powi(2)
                + noise_value.derivative[1].powi(2))
            .sqrt();

            total_flatness += 1. / magnitude;

            let gradience = self.get_gradient_influence(total_flatness);

            // Add the signal to the result.
            result = result + noise_value * (gradience * amplitude);
        }

        // Scale the result into the [-1,1] range
        result * self.scale_factor
    }
}
