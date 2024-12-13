use super::{Parser, ParserResult};

pub fn nothing<IT: Iterator>(input: IT) -> ParserResult<IT, ()> {
    Some((input, ()))
}

pub fn peek<IT: Iterator + Clone, T>(
    mut p: impl Parser<IT, T>,
) -> impl FnMut(IT) -> ParserResult<IT, T> {
    move |input: IT| {
        let cloned = input.clone();
        p(cloned).map(|(_, v)| (input, v))
    }
}

pub fn tag<IT: Iterator<Item = char>>(c: char) -> impl FnMut(IT) -> ParserResult<IT, ()> {
    move |mut input: IT| {
        let first = input.next()?;
        (first == c).then_some((input, ()))
    }
}

pub fn anything<IT: Iterator<Item = T>, T>(mut input: IT) -> ParserResult<IT, T> {
    let next = input.next()?;
    Some((input, next))
}

pub fn ascii_digit<IT: Iterator<Item = char>>(input: IT) -> ParserResult<IT, char> {
    anything.and_then(nothing, |c: char, _| c.is_ascii_digit().then_some(c))(input)
}
