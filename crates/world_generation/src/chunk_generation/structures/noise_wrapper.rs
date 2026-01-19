use std::{fmt::Debug, sync::Arc};

use bevy::prelude::Deref;
use rand::{SeedableRng, rngs::StdRng};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
};

use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
    terrain_noise::TerrainNoise,
};

#[derive(Clone, Deref)]
pub struct NoiseWrapper {
    pub noise_map: TerrainNoise,
    pub seed: u64,
    #[deref]
    pub noise: Arc<Box<dyn NoiseFunction<NoiseResult, [f64; 2]> + Send + Sync>>,
}

impl Debug for NoiseWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoiseWrapper")
            .field("noise_map", &self.noise_map)
            .field("seed", &self.seed)
            .finish()
    }
}

impl NoiseWrapper {
    pub fn new(noise_map: TerrainNoise, seed: u64) -> Self {
        Self {
            noise: Arc::new(
                noise_map.get_noise_fn(&mut StdRng::seed_from_u64(seed)),
            ),
            noise_map,
            seed,
        }
    }
}

impl<'de> Deserialize<'de> for NoiseWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            NoiseMap,
            Seed,
        }

        struct NoiseWrapperVisitor;

        impl<'de> Visitor<'de> for NoiseWrapperVisitor {
            type Value = NoiseWrapper;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("struct NoiseWrapper")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<NoiseWrapper, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let noise_map = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let seed = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(NoiseWrapper::new(noise_map, seed))
            }

            fn visit_map<V>(self, mut map: V) -> Result<NoiseWrapper, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut noise_map = None;
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::NoiseMap => {
                            if noise_map.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "noise_map",
                                ));
                            }
                            noise_map = Some(map.next_value()?);
                        }
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(de::Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        }
                    }
                }
                let noise_map = noise_map
                    .ok_or_else(|| de::Error::missing_field("noise_map"))?;
                let seed =
                    seed.ok_or_else(|| de::Error::missing_field("seed"))?;
                Ok(NoiseWrapper::new(noise_map, seed))
            }
        }

        const FIELDS: &[&str] = &["noise_map", "seed"];
        deserializer.deserialize_struct(
            "NoiseWrapper",
            FIELDS,
            NoiseWrapperVisitor,
        )
    }
}

impl Serialize for NoiseWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("NoiseWrapper", 3)?;
        state.serialize_field("noise_map", &self.noise_map)?;
        state.serialize_field("seed", &self.seed)?;
        state.end()
    }
}
