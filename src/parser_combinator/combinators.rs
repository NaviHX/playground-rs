use super::{Parser, ParserResult};

pub fn id<T>(t: T) -> T {
    t
}

pub fn map_parsed<IT: Iterator, T, S>(
    p: impl Parser<IT, T>,
    f: impl FnMut(T) -> S,
) -> impl FnMut(IT) -> ParserResult<IT, S> {
    p.map(f)
}

pub fn and<IT: Iterator, T1, T2, R, F>(
    p1: impl Parser<IT, T1>,
    p2: impl Parser<IT, T2>,
    f: F,
) -> impl FnMut(IT) -> ParserResult<IT, R>
where
    F: FnMut(T1, T2) -> Option<R>,
{
    p1.and_then(p2, f)
}

pub fn or<IT, T, R, F>(
    p1: impl Parser<IT, T>,
    p2: impl Parser<IT, T>,
    f: F,
) -> impl FnMut(IT) -> ParserResult<IT, R>
where
    F: FnMut(T) -> R,
    IT: Iterator + Clone,
{
    map_parsed(p1.or_else(p2), f)
}

fn nothing<IT: Iterator, T>(input: IT) -> ParserResult<IT, Option<T>> {
    Some((input, None))
}

fn wrap<IT: Iterator, T>(
    mut p: impl Parser<IT, T>,
) -> impl FnMut(IT) -> ParserResult<IT, Option<T>> {
    move |input: IT| p(input).map(|(input, v)| (input, Some(v)))
}

pub fn opt<IT: Iterator + Clone, T>(
    p: impl Parser<IT, T>,
) -> impl FnMut(IT) -> ParserResult<IT, Option<T>> {
    let pp = wrap(p);
    or(pp, nothing, id)
}

pub fn many0<IT, T, R, F>(
    mut p: impl Parser<IT, T>,
    mut f: F,
    init: R,
) -> impl FnMut(IT) -> ParserResult<IT, R>
where
    R: Clone,
    F: FnMut(R, T) -> R,
    IT: Iterator + Clone,
{
    move |mut input: IT| {
        let mut fold = init.clone();
        while let Some((s, v)) = p(input.clone()) {
            input = s;
            fold = f(fold, v);
        }

        Some((input, fold))
    }
}

pub fn many<IT, R, F>(mut p: impl Parser<IT, R>, mut f: F) -> impl FnMut(IT) -> ParserResult<IT, R>
where
    F: FnMut(R, R) -> R,
    IT: Iterator + Clone,
{
    move |input: IT| {
        let (mut input, mut fold) = p(input)?;
        while let Some((s, v)) = p(input.clone()) {
            input = s;
            fold = f(fold, v);
        }

        Some((input, fold))
    }
}
