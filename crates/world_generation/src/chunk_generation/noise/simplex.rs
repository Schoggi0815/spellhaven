use noise::{core::simplex::simplex_2d, permutationtable::PermutationTable};

use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct Simplex {
    seed: u32,
    hasher: PermutationTable,
}

impl Simplex {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            hasher: PermutationTable::new(seed),
        }
    }

    pub fn get_seed(&self) -> u32 {
        self.seed
    }
}

impl NoiseFunction<NoiseResult, [f64; 2]> for Simplex {
    fn get(&self, input: [f64; 2]) -> NoiseResult {
        let (value, derivative) = simplex_2d(input.into(), &self.hasher);

        NoiseResult {
            value,
            derivative: [derivative[0] * 4., derivative[1] * 4.],
        }
    }
}
