// rationel :: [sign] (dot_decimal | decimal) [exp]
// dot_decimal :: '.' uint
// decimal :: uint [dot_with_opt_decimal]
// dot_with_opt_decimal :: '.' [uint]
// exp :: ('e' | 'E') [sign] uint
// sign :: '+' | '-'
// uint :: ascii_digit+

use playground_rs::parser_combinator::combinators::{many, opt, or};
use playground_rs::parser_combinator::utils::{ascii_digit, tag};
use playground_rs::parser_combinator::{Parser, ParserResult};

fn discard<T>(_: T) {}
fn discard_both<T, S>(_: T, _: S) {}
fn and_then_discard_both<T, S>(_: T, _: S) -> Result<(), ()> {
    Ok(())
}

fn rationel<IT: Iterator<Item = char> + Clone>(input: IT) -> ParserResult<IT, (), ()> {
    let dot_decimal = dot().and_then(uint, and_then_discard_both);
    let dot_with_opt_decimal = dot().and_then(opt(uint), and_then_discard_both);
    let decimal = uint.and_then(opt(dot_with_opt_decimal), and_then_discard_both);
    let exp = (tag('e').or_else(tag('E'))).map_err(|_| ())
        .and_then(opt(sign), and_then_discard_both)
        .and_then(uint, and_then_discard_both);

    opt(sign)
        .and_then(dot_decimal.or_else(decimal), and_then_discard_both)
        .and_then(opt(exp), and_then_discard_both)(input)
}

fn sign<IT: Iterator<Item = char> + Clone>(input: IT) -> ParserResult<IT, (), ()> {
    or(tag('-'), tag('+'), discard).map_err(|_| ())(input)
}

fn uint<IT: Iterator<Item = char> + Clone>(input: IT) -> ParserResult<IT, (), ()> {
    many(ascii_digit.map(discard), discard_both).map_err(|_| ())(input)
}

fn dot<IT: Iterator<Item = char>>() -> impl FnMut(IT) -> ParserResult<IT, (), ()> {
    tag('.').map_err(|_| ())
}

fn main() {
    let input = "1";
    println!(
        "'{input}' {} a rationel number",
        if rationel(input.chars())
            .map(|mut buf| buf.0.next().is_none())
            .is_ok()
        {
            "is"
        } else {
            "isn't"
        }
    );
}
