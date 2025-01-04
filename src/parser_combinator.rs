pub mod combinators;
pub mod utils;

pub type ParserResult<IT, T, E> = Result<(IT, T), (IT, E)>;
pub trait Parser<IT: Iterator, T, E>: FnMut(IT) -> ParserResult<IT, T, E> + Sized {
    fn and_then<S, R>(
        mut self,
        mut p: impl Parser<IT, S, E>,
        mut f: impl FnMut(T, S) -> Result<R, E>,
    ) -> impl FnMut(IT) -> ParserResult<IT, R, E> {
        move |input: IT| {
            let (input, vs) = self(input)?;
            let (input, vp) = p(input)?;
            match f(vs, vp) {
                Ok(vf) => Ok((input, vf)),
                Err(ef) => Err((input, ef)),
            }
        }
    }

    fn or_else(mut self, mut p: impl Parser<IT, T, E>) -> impl FnMut(IT) -> ParserResult<IT, T, E>
    where
        IT: Clone,
    {
        move |input: IT| self(input.clone()).or_else(|(_, _)| p(input))
    }

    fn map<S>(mut self, mut f: impl FnMut(T) -> S) -> impl FnMut(IT) -> ParserResult<IT, S, E> {
        move |input: IT| self(input).map(|(input, v)| (input, f(v)))
    }

    fn map_err<E2>(mut self, mut f: impl FnMut(E) -> E2) -> impl FnMut(IT) -> ParserResult<IT, T, E2> {
        move |input: IT| self(input).map_err(|(input, e)| (input, f(e)))
    }
}

impl<T: FnMut(IT) -> ParserResult<IT, R, E>, R, E, IT: Iterator> Parser<IT, R, E> for T {}
