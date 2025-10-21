use std::{cell::RefCell, collections::HashMap};

use crate::chunk_generation::noise::{
    noise_function::NoiseFunction, noise_result::NoiseResult,
};

pub struct FullCache<T> {
    source: T,
    cache_map: RefCell<HashMap<[i64; 2], NoiseResult>>,
}

impl<T> FullCache<T> {
    pub fn new(source: T) -> Self {
        Self {
            source,
            cache_map: RefCell::new(HashMap::new()),
        }
    }
}

impl<T> Default for FullCache<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            source: Default::default(),
            cache_map: RefCell::new(HashMap::new()),
        }
    }
}

impl<T> NoiseFunction<NoiseResult, [f64; 2]> for FullCache<T>
where
    T: NoiseFunction<NoiseResult, [f64; 2]>,
{
    fn get(&self, input: [f64; 2]) -> NoiseResult {
        let cache_key = [input[0] as i64, input[1] as i64];
        let mut map = self.cache_map.borrow_mut();
        let item = map.get(&cache_key);
        if let Some(cache_value) = item {
            return *cache_value;
        }
        let value = self.source.get(input);
        map.insert(cache_key, value);
        return value;
    }
}
