use super::{Parser, ParserResult};

pub fn id<T>(t: T) -> T {
    t
}

pub fn map_parsed<T, S>(
    p: impl Parser<T>,
    f: impl FnMut(T) -> S,
) -> impl FnMut(&str) -> ParserResult<S> {
    p.map(f)
}

pub fn and<T1, T2, R, F>(
    p1: impl Parser<T1>,
    p2: impl Parser<T2>,
    f: F,
) -> impl FnMut(&str) -> ParserResult<R>
where
    F: FnMut(T1, T2) -> Option<R>,
{
    p1.and_then(p2, f)
}

pub fn or<T, R, F>(
    p1: impl Parser<T>,
    p2: impl Parser<T>,
    f: F,
) -> impl FnMut(&str) -> ParserResult<R>
where
    F: FnMut(T) -> R,
{
    map_parsed(p1.or_else(p2), f)
}

pub fn opt<T>(p: impl Parser<T>) -> impl FnMut(&str) -> ParserResult<Option<T>> {
    fn nothing<T>(input: &str) -> ParserResult<Option<T>> {
        Some((input, None))
    }

    fn wrap<T>(mut p: impl Parser<T>) -> impl FnMut(&str) -> ParserResult<Option<T>> {
        move |input: &str| p(input).map(|(input, v)| (input, Some(v)))
    }

    let pp = wrap(p);
    or(pp, nothing, id)
}

pub fn many0<T, R, F>(
    mut p: impl Parser<T>,
    mut f: F,
    init: R,
) -> impl FnMut(&str) -> ParserResult<R>
where
    R: Clone,
    F: FnMut(R, T) -> R,
{
    move |mut input: &str| {
        let mut fold = init.clone();
        while let Some((s, v)) = p(input) {
            input = s;
            fold = f(fold, v);
        }

        Some((input, fold))
    }
}

pub fn many<R, F>(mut p: impl Parser<R>, mut f: F) -> impl FnMut(&str) -> ParserResult<R>
where
    F: FnMut(R, R) -> R,
{
    move |input: &str| {
        let (mut input, mut fold) = p(input)?;
        while let Some((s, v)) = p(input) {
            input = s;
            fold = f(fold, v);
        }

        Some((input, fold))
    }
}
