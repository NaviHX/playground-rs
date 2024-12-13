pub mod combinators;
pub mod utils;

pub type ParserResult<'a, T> = Option<(&'a str, T)>;
pub trait Parser<T>: FnMut(&str) -> ParserResult<T> + Sized {
    fn and_then<S, R>(
        mut self,
        mut p: impl Parser<S>,
        mut f: impl FnMut(T, S) -> Option<R>,
    ) -> impl FnMut(&str) -> ParserResult<R> {
        move |input: &str| {
            self(input).and_then(|(input, vs)| {
                p(input).and_then(|(input, vp)| f(vs, vp).map(|v| (input, v)))
            })
        }
    }

    fn or_else(mut self, mut p: impl Parser<T>) -> impl FnMut(&str) -> ParserResult<T> {
        move |input: &str| self(input).or_else(|| p(input))
    }

    fn map<S>(mut self, mut f: impl FnMut(T) -> S) -> impl FnMut(&str) -> ParserResult<S> {
        move |input: &str| self(input).map(|(input, v)| (input, f(v)))
    }
}

impl<T: FnMut(&str) -> ParserResult<R>, R> Parser<R> for T {}
