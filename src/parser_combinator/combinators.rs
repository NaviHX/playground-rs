use super::{Parser, ParserResult};

pub fn id<T>(t: T) -> T {
    t
}

pub fn map_parsed<IT: Iterator, T, S, E>(
    p: impl Parser<IT, T, E>,
    f: impl FnMut(T) -> S,
) -> impl FnMut(IT) -> ParserResult<IT, S, E> {
    p.map(f)
}

pub fn and<IT: Iterator, T1, T2, R, F, E>(
    p1: impl Parser<IT, T1, E>,
    p2: impl Parser<IT, T2, E>,
    f: F,
) -> impl FnMut(IT) -> ParserResult<IT, R, E>
where
    F: FnMut(T1, T2) -> Result<R, E>,
{
    p1.and_then(p2, f)
}

pub fn or<IT, T, R, F, E>(
    p1: impl Parser<IT, T, E>,
    p2: impl Parser<IT, T, E>,
    f: F,
) -> impl FnMut(IT) -> ParserResult<IT, R, E>
where
    F: FnMut(T) -> R,
    IT: Iterator + Clone,
{
    map_parsed(p1.or_else(p2), f)
}

fn nothing<IT: Iterator, E>(input: IT) -> ParserResult<IT, (), E> {
    Ok((input, ()))
}

fn wrap<IT: Iterator, T, E>(
    mut p: impl Parser<IT, T, E>,
) -> impl FnMut(IT) -> ParserResult<IT, Option<T>, E> {
    move |input: IT| p(input).map(|(input, v)| (input, Some(v)))
}

pub fn opt<IT: Iterator + Clone, T, E>(
    p: impl Parser<IT, T, E>,
) -> impl FnMut(IT) -> ParserResult<IT, Option<T>, E> {
    let p = wrap(p);
    let nothing = map_parsed(nothing, |_| None);
    or(p, nothing, id)
}

pub fn many0<IT, T, R, F, E>(
    mut p: impl Parser<IT, T, E>,
    mut f: F,
    init: R,
) -> impl FnMut(IT) -> ParserResult<IT, R, E>
where
    R: Clone,
    F: FnMut(R, T) -> R,
    IT: Iterator + Clone,
{
    move |mut input: IT| {
        let mut fold = init.clone();
        while let Ok((s, v)) = p(input.clone()) {
            input = s;
            fold = f(fold, v);
        }

        Ok((input, fold))
    }
}

pub fn many<IT, R, F, E>(mut p: impl Parser<IT, R, E>, mut f: F) -> impl FnMut(IT) -> ParserResult<IT, R, E>
where
    F: FnMut(R, R) -> R,
    IT: Iterator + Clone,
{
    move |input: IT| {
        let (mut input, mut fold) = p(input)?;
        while let Ok((s, v)) = p(input.clone()) {
            input = s;
            fold = f(fold, v);
        }

        Ok((input, fold))
    }
}

pub fn many_mn<IT, T, R, F, E>(
    mut p: impl Parser<IT, T, E>,
    m: usize,
    n: usize,
    mut f: F,
    init: R,
) -> impl FnMut(IT) -> ParserResult<IT, R, (usize, R)>
where
    R: Clone,
    F: FnMut(R, T) -> R,
    IT: Iterator + Clone,
{
    assert!(m <= n);

    move |mut input: IT| {
        if n == 0 {
            return Ok((input, init.clone()));
        }

        let mut fold = init.clone();
        let mut count = 0;
        while let Ok((s, v)) = p(input.clone()) {
            input = s;
            fold = f(fold, v);
            count += 1;
            if count == n {
                break;
            }
        }

        if count >= m {
            Ok((input, fold))
        } else {
            Err((input, (count, fold)))
        }
    }
}

pub fn many_till<IT, T, C, R, F, E>(
    mut p: impl Parser<IT, T, E>,
    mut cond: impl Parser<IT, C, E>,
    mut f: F,
    init: R,
) -> impl FnMut(IT) -> ParserResult<IT, (R, Option<C>), E>
where
    R: Clone,
    F: FnMut(R, T) -> R,
    IT: Iterator + Clone,
{
    move |mut input: IT| {
        let mut fold = init.clone();
        loop {
            let c = cond(input.clone());
            if let Ok((remainder, c)) = c {
                break Ok((remainder, (fold, Some(c))));
            }

            let r = p(input.clone());
            if let Ok((remainder, r)) = r {
                input = remainder;
                fold = f(fold, r);
                continue;
            }

            break Ok((input, (fold, None)));
        }
    }
}
