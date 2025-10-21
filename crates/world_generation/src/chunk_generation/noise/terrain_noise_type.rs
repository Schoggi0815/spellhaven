use bevy::log::warn;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::chunk_generation::{
    VOXEL_SIZE,
    noise::{
        abs::Abs, add::Add, constant::Constant, gradient_fractal_noise::GFT,
        max::Max, multiply::Multiply, negate::Negate,
        noise_function::NoiseFunction, noise_result::NoiseResult,
        scale_point::ScalePoint, simplex::Simplex, smooth_step::SmoothStep,
        translate_point::TranslatePoint,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TerrainNoiseType {
    Simplex {
        seed_index: usize,
    },
    Constant {
        value_index: usize,
    },
    Add {
        a_index: usize,
        b_index: usize,
    },
    Sub {
        a_index: usize,
        b_index: usize,
    },
    Max {
        a_index: usize,
        b_index: usize,
    },
    Abs {
        input_index: usize,
    },
    Multiply {
        a_index: usize,
        b_index: usize,
    },
    SmoothStep {
        noise_index: usize,
        steps_index: usize,
        smoothness_index: usize,
    },
    ScalePoint {
        noise_index: usize,
        scale_index: usize,
    },
    TranslatePoint {
        noise_index: usize,
        x_index: usize,
        y_index: usize,
    },
    GFT {
        noise_index: usize,
        octaves_index: usize,
        frequency_index: usize,
        lacunarity_index: usize,
        persistence_index: usize,
        gradient_index: usize,
        amplitude_index: usize,
    },
    ConstantValue {
        value: ConstantValue,
    },
    RandomI64,
    RandomF64 {
        min_index: usize,
        max_index: usize,
    },
    Powf64 {
        a_index: usize,
        b_index: usize,
    },
    Dividef64 {
        a_index: usize,
        b_index: usize,
    },
    VoxelSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConstantValue {
    F64(f64),
    I64(i64),
}

impl ConstantValue {
    pub fn get_f64(&self) -> f64 {
        match self {
            ConstantValue::F64(value) => *value,
            _ => 0.,
        }
    }

    pub fn get_i64(&self) -> i64 {
        match self {
            ConstantValue::I64(value) => *value,
            _ => 0,
        }
    }
}

impl TerrainNoiseType {
    pub fn to_noise_fn(
        &self,
        noise_types: &Vec<TerrainNoiseType>,
        rng: &mut impl Rng,
    ) -> Box<dyn NoiseFunction<NoiseResult, [f64; 2]> + Send + Sync> {
        match self {
            TerrainNoiseType::Simplex { seed_index } => Box::new(Simplex::new(
                noise_types[*seed_index].to_i64_value(noise_types, rng) as u32,
            )),
            TerrainNoiseType::Add { a_index, b_index } => Box::new(Add::new(
                noise_types[*a_index].to_noise_fn(noise_types, rng),
                noise_types[*b_index].to_noise_fn(noise_types, rng),
            )),
            TerrainNoiseType::Sub { a_index, b_index } => Box::new(Add::new(
                noise_types[*a_index].to_noise_fn(noise_types, rng),
                Negate::new(
                    noise_types[*b_index].to_noise_fn(noise_types, rng),
                ),
            )),
            TerrainNoiseType::Constant { value_index } => {
                Box::new(Constant::new(
                    noise_types[*value_index].to_f64_value(noise_types, rng),
                ))
            }
            TerrainNoiseType::Max { a_index, b_index } => Box::new(Max::new(
                noise_types[*a_index].to_noise_fn(noise_types, rng),
                noise_types[*b_index].to_noise_fn(noise_types, rng),
            )),
            TerrainNoiseType::Abs { input_index } => Box::new(Abs::new(
                noise_types[*input_index].to_noise_fn(noise_types, rng),
            )),
            TerrainNoiseType::Multiply { a_index, b_index } => {
                Box::new(Multiply::new(
                    noise_types[*a_index].to_noise_fn(noise_types, rng),
                    noise_types[*b_index].to_noise_fn(noise_types, rng),
                ))
            }
            TerrainNoiseType::SmoothStep {
                noise_index,
                steps_index,
                smoothness_index,
            } => Box::new(
                SmoothStep::new(
                    noise_types[*noise_index].to_noise_fn(noise_types, rng),
                )
                .set_smoothness(
                    noise_types[*smoothness_index]
                        .to_f64_value(noise_types, rng),
                )
                .set_steps(
                    noise_types[*steps_index].to_f64_value(noise_types, rng),
                ),
            ),
            TerrainNoiseType::ScalePoint {
                noise_index,
                scale_index,
            } => Box::new(ScalePoint::new(
                noise_types[*noise_index].to_noise_fn(noise_types, rng),
                noise_types[*scale_index].to_f64_value(noise_types, rng),
            )),
            TerrainNoiseType::TranslatePoint {
                noise_index,
                x_index,
                y_index,
            } => Box::new(TranslatePoint::new(
                noise_types[*noise_index].to_noise_fn(noise_types, rng),
                noise_types[*x_index].to_f64_value(noise_types, rng),
                noise_types[*y_index].to_f64_value(noise_types, rng),
            )),
            TerrainNoiseType::GFT {
                noise_index,
                octaves_index,
                frequency_index,
                lacunarity_index,
                persistence_index,
                gradient_index,
                amplitude_index,
            } => Box::new(
                GFT::new_with_source(
                    noise_types[*noise_index].to_noise_fn(noise_types, rng),
                )
                .set_octaves(
                    noise_types[*octaves_index].to_i64_value(noise_types, rng)
                        as usize,
                )
                .set_frequency(
                    noise_types[*frequency_index]
                        .to_f64_value(noise_types, rng),
                )
                .set_lacunarity(
                    noise_types[*lacunarity_index]
                        .to_f64_value(noise_types, rng),
                )
                .set_persistence(
                    noise_types[*persistence_index]
                        .to_f64_value(noise_types, rng),
                )
                .set_gradient(
                    noise_types[*gradient_index].to_f64_value(noise_types, rng),
                )
                .set_amplitude(
                    noise_types[*amplitude_index]
                        .to_f64_value(noise_types, rng),
                ),
            ),
            _ => Box::new(Constant::new(0.)),
        }
    }

    fn to_f64_value(
        &self,
        noise_types: &Vec<TerrainNoiseType>,
        rng: &mut impl Rng,
    ) -> f64 {
        match self {
            TerrainNoiseType::ConstantValue { value } => value.get_f64(),
            TerrainNoiseType::Powf64 { a_index, b_index } => {
                let a = noise_types[*a_index].to_f64_value(noise_types, rng);
                let b = noise_types[*b_index].to_f64_value(noise_types, rng);
                a.powf(b)
            }
            TerrainNoiseType::Dividef64 { a_index, b_index } => {
                let a = noise_types[*a_index].to_f64_value(noise_types, rng);
                let b = noise_types[*b_index].to_f64_value(noise_types, rng);
                a / b
            }
            TerrainNoiseType::VoxelSize => VOXEL_SIZE as f64,
            TerrainNoiseType::RandomF64 {
                min_index,
                max_index,
            } => {
                let min =
                    noise_types[*min_index].to_f64_value(noise_types, rng);
                let max =
                    noise_types[*max_index].to_f64_value(noise_types, rng);
                rng.random_range(min..max)
            }
            _ => {
                warn!("ZERO f64");
                0.
            }
        }
    }

    fn to_i64_value(
        &self,
        _noise_types: &Vec<TerrainNoiseType>,
        rng: &mut impl Rng,
    ) -> i64 {
        match self {
            TerrainNoiseType::ConstantValue { value } => value.get_i64(),
            TerrainNoiseType::RandomI64 => rng.random(),
            _ => {
                warn!("ZERO i64");
                0
            }
        }
    }
}
