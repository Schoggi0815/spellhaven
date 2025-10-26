use ron::ser::PrettyConfig;
use world_generation::chunk_generation::{
    noise::{
        terrain_noise::TerrainNoise,
        terrain_noise_type::{ConstantValue, TerrainNoiseType},
    },
    structures::noise_wrapper::NoiseWrapper,
};

pub fn main() {
    let noise = NoiseWrapper::new(
        TerrainNoise::new(
            0,
            vec![
                TerrainNoiseType::Constant { value_index: 1 },
                TerrainNoiseType::RandomF64 {
                    min_index: 3,
                    max_index: 2,
                },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(10.),
                },
                TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(-10.),
                },
            ],
        ),
        1234,
    );

    println!("BEFORE: {}", noise.get([0., 0.]));

    let ron_string =
        ron::ser::to_string_pretty(&noise, PrettyConfig::default()).unwrap();

    println!("NOISE RON: {}", ron_string);

    let noise: NoiseWrapper = ron::from_str(&ron_string).unwrap();

    println!("AFTER: {}", noise.get([0., 0.]));
}
