pub mod combinators;
pub mod utils;

pub type ParserResult<IT, T> = Option<(IT, T)>;
pub trait Parser<IT: Iterator, T>: FnMut(IT) -> ParserResult<IT, T> + Sized {
    fn and_then<S, R>(
        mut self,
        mut p: impl Parser<IT, S>,
        mut f: impl FnMut(T, S) -> Option<R>,
    ) -> impl FnMut(IT) -> ParserResult<IT, R> {
        move |input: IT| {
            self(input).and_then(|(input, vs)| {
                p(input).and_then(|(input, vp)| f(vs, vp).map(|v| (input, v)))
            })
        }
    }

    fn or_else(mut self, mut p: impl Parser<IT, T>) -> impl FnMut(IT) -> ParserResult<IT, T>
    where
        IT: Clone,
    {
        move |input: IT| self(input.clone()).or_else(|| p(input))
    }

    fn map<S>(mut self, mut f: impl FnMut(T) -> S) -> impl FnMut(IT) -> ParserResult<IT, S> {
        move |input: IT| self(input).map(|(input, v)| (input, f(v)))
    }
}

impl<T: FnMut(IT) -> ParserResult<IT, R>, R, IT: Iterator> Parser<IT, R> for T {}
