use super::{Parser, ParserResult};

pub fn nothing(input: &str) -> ParserResult<()> {
    Some((input, ()))
}

pub fn peek<T>(mut p: impl Parser<T>) -> impl FnMut(&str) -> ParserResult<T> {
    move |input: &str| p(input).map(|(_, t)| (input, t))
}

pub fn tag(c: char) -> impl FnMut(&str) -> ParserResult<()> {
    move |input: &str| input.strip_prefix(c).map(|remainder| (remainder, ()))
}

pub fn any_char(input: &str) -> ParserResult<char> {
    input.chars().next().map(|c| (&input[1..], c))
}

pub fn ascii_digit(input: &str) -> ParserResult<char> {
    any_char.and_then(nothing, |c, _| c.is_ascii_digit().then_some(c))(input)
}
