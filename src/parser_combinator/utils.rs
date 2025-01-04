use super::{Parser, ParserResult};

pub fn nothing<IT: Iterator, E>(input: IT) -> ParserResult<IT, (), E> {
    Ok((input, ()))
}

pub fn peek<IT: Iterator + Clone, T, E>(
    mut p: impl Parser<IT, T, E>,
) -> impl FnMut(IT) -> ParserResult<IT, T, E> {
    move |input: IT| {
        let cloned = input.clone();
        p(cloned).map(|(_, v)| (input, v))
    }
}

pub fn tag<IT: Iterator>(c: IT::Item) -> impl FnMut(IT) -> ParserResult<IT, (), Option<IT::Item>>
where
    IT::Item: Eq,
{
    anything.map_err(|_| None).and_then(
        nothing,
        move |item: IT::Item, _| {
            if item == c {
                Ok(())
            } else {
                Err(Some(item))
            }
        },
    )
}

pub fn anything<IT: Iterator<Item = T>, T>(mut input: IT) -> ParserResult<IT, T, ()> {
    match input.next() {
        Some(item) => Ok((input, item)),
        None => Err((input, ())),
    }
}

pub fn ascii_digit<IT: Iterator<Item = char>>(input: IT) -> ParserResult<IT, char, Option<char>> {
    anything.map_err(|_| None).and_then(nothing, |c: char, _| {
        if c.is_ascii_digit() {
            Ok(c)
        } else {
            Err(Some(c))
        }
    })(input)
}
