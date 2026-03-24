use std::ops::Add;

use bevy::math::Vec2;
use noiz::NoiseFunction;

pub struct AddN<T1, T2> {
    source_1: T1,
    source_2: T2,
}

impl<T1, T2> AddN<T1, T2> {
    pub fn new(source_1: T1, source_2: T2) -> Self {
        Self { source_1, source_2 }
    }
}

impl<T1: NoiseFunction<Vec2, Output: Add<T2::Output>>, T2: NoiseFunction<Vec2>>
    NoiseFunction<Vec2> for AddN<T1, T2>
{
    type Output = <T1::Output as Add<T2::Output>>::Output;

    fn evaluate(
        &self,
        input: Vec2,
        seeds: &mut noiz::rng::NoiseRng,
    ) -> Self::Output {
        self.source_1.evaluate(input, seeds)
            + self.source_2.evaluate(input, seeds)
    }
}
