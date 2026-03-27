use bevy::math::Vec2;
use noiz::{NoiseFunction, cells::WithGradient};

pub struct Max<T1, T2> {
    source_1: T1,
    source_2: T2,
}

impl<T1, T2> Max<T1, T2> {
    pub fn new(source_1: T1, source_2: T2) -> Self {
        Self { source_1, source_2 }
    }
}

impl<T1: NoiseFunction<Vec2, Output = WithGradient<f32, Vec2>>>
    NoiseFunction<Vec2> for Max<T1, T1>
{
    type Output = T1::Output;

    fn evaluate(
        &self,
        input: Vec2,
        seeds: &mut noiz::rng::NoiseRng,
    ) -> Self::Output {
        let ev1 = self.source_1.evaluate(input, seeds);
        let ev2 = self.source_2.evaluate(input, seeds);
        if ev1.value > ev2.value { ev1 } else { ev2 }
    }
}
