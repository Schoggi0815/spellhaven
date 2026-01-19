use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct TranslatePoint<T> {
    source: T,
    x_translation: f64,
    y_translation: f64,
}

impl<T> TranslatePoint<T> {
    pub fn new(source: T, x_translation: f64, y_translation: f64) -> Self {
        Self {
            source,
            x_translation,
            y_translation,
        }
    }
}

impl<T> NoiseFunction<NoiseResult, [f64; 2]> for TranslatePoint<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    fn get(&self, input: [f64; 2]) -> NoiseResult {
        self.source
            .get([input[0] + self.x_translation, input[1] + self.y_translation])
    }
}
