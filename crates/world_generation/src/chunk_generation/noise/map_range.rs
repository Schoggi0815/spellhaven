use noise::NoiseFn;

pub struct MapRange<
    TBaseNoise,
    TFromMinNoise,
    TFromMaxNoise,
    TToMinNoise,
    TToMaxNoise,
> {
    noise: TBaseNoise,
    from_min: TFromMinNoise,
    from_max: TFromMaxNoise,
    to_min: TToMinNoise,
    to_max: TToMaxNoise,
}

impl<TBaseNoise, TFromMinNoise, TFromMaxNoise, TToMinNoise, TToMaxNoise>
    MapRange<TBaseNoise, TFromMinNoise, TFromMaxNoise, TToMinNoise, TToMaxNoise>
{
    pub fn new(
        noise: TBaseNoise,
        from_min: TFromMinNoise,
        from_max: TFromMaxNoise,
        to_min: TToMinNoise,
        to_max: TToMaxNoise,
    ) -> Self {
        Self {
            noise,
            from_min,
            from_max,
            to_min,
            to_max,
        }
    }
}

impl<
    TBaseNoise,
    TFromMinNoise,
    TFromMaxNoise,
    TToMinNoise,
    TToMaxNoise,
    const D: usize,
> NoiseFn<f64, D>
    for MapRange<
        TBaseNoise,
        TFromMinNoise,
        TFromMaxNoise,
        TToMinNoise,
        TToMaxNoise,
    >
where
    TBaseNoise: NoiseFn<f64, D>,
    TFromMinNoise: NoiseFn<f64, D>,
    TFromMaxNoise: NoiseFn<f64, D>,
    TToMinNoise: NoiseFn<f64, D>,
    TToMaxNoise: NoiseFn<f64, D>,
{
    fn get(&self, point: [f64; D]) -> f64 {
        let from_min = self.from_min.get(point);
        let from_max = self.from_max.get(point);
        let to_min = self.to_min.get(point);
        let to_max = self.to_max.get(point);

        let mut value = self.noise.get(point);

        value -= from_min;

        let scale = (to_max - to_min) / (from_max - from_min);
        value *= scale;
        value += to_min;
        value
    }
}
