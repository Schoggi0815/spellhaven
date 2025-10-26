use noise::NoiseFn;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::chunk_generation::noise::terrain_noise_type::{
    ConstantValue, TerrainNoiseType,
};

pub const TERRAIN_NOISE_FILE_PATH: &'static str = "assets/terrain_noise.ron";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerrainNoise {
    noise_types: Vec<TerrainNoiseType>,
    start_index: usize,
}

impl TerrainNoise {
    pub fn new(start_index: usize, noise_types: Vec<TerrainNoiseType>) -> Self {
        Self {
            noise_types,
            start_index,
        }
    }

    pub fn get_noise_fn(
        &self,
        rng: &mut impl Rng,
    ) -> Box<dyn NoiseFn<f64, 2> + Send + Sync> {
        self.noise_types[self.start_index].to_noise_fn(&self.noise_types, rng)
    }

    pub fn new_basic_simplex(
        frequency: f64,
        min_value: f64,
        max_value: f64,
    ) -> Self {
        Self {
            start_index: 14,
            noise_types: vec![
                TerrainNoiseType::Simplex { seed_index: 1 },
                TerrainNoiseType::RandomI64,
                TerrainNoiseType::Add {
                    a_index: 0,
                    b_index: 3,
                },
                TerrainNoiseType::Constant { value_index: 4 },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(1.),
                },
                TerrainNoiseType::Multiply {
                    a_index: 2,
                    b_index: 6,
                },
                TerrainNoiseType::Constant { value_index: 7 },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(0.5),
                },
                TerrainNoiseType::Multiply {
                    a_index: 5,
                    b_index: 9,
                },
                TerrainNoiseType::Constant { value_index: 10 },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(max_value - min_value),
                },
                TerrainNoiseType::Add {
                    a_index: 8,
                    b_index: 12,
                },
                TerrainNoiseType::Constant { value_index: 13 },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(min_value),
                },
                TerrainNoiseType::ScalePoint {
                    noise_index: 11,
                    scale_index: 15,
                },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(frequency),
                },
            ],
        }
    }
}
