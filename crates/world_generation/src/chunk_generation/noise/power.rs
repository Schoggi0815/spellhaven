use bevy::math::Vec2;
use noiz::{NoiseFunction, cells::WithGradient};

pub struct Power<T1> {
    source: T1,
    exponent: f32,
}

impl<T1> Power<T1> {
    pub fn new(source: T1, exponent: f32) -> Self {
        Self { source, exponent }
    }
}

impl<T1: NoiseFunction<Vec2, Output = WithGradient<f32, Vec2>>>
    NoiseFunction<Vec2> for Power<T1>
{
    type Output = WithGradient<f32, Vec2>;

    fn evaluate(
        &self,
        input: Vec2,
        seeds: &mut noiz::rng::NoiseRng,
    ) -> Self::Output {
        let source = self.source.evaluate(input, seeds);

        WithGradient {
            value: source.value.powf(self.exponent),
            gradient: source.gradient
                * (self.exponent * source.value.powf(self.exponent - 1.)),
        }
    }
}
