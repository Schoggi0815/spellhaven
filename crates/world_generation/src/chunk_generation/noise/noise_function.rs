pub trait NoiseFunction<TResult, TInput> {
    fn get(&self, input: TInput) -> TResult;
}

impl<'a, TResult, TInput, M> NoiseFunction<TResult, TInput> for &'a M
where
    M: NoiseFunction<TResult, TInput> + ?Sized,
{
    #[inline]
    fn get(&self, point: TInput) -> TResult {
        M::get(*self, point)
    }
}

impl<TResult, TInput, M> NoiseFunction<TResult, TInput> for Box<M>
where
    M: NoiseFunction<TResult, TInput> + ?Sized,
{
    #[inline]
    fn get(&self, point: TInput) -> TResult {
        M::get(self, point)
    }
}
