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
fn and_then_discard_both<T, S>(_: T, _: S) -> Option<()> {
    Some(())
}

fn rationel(input: &str) -> ParserResult<()> {
    let dot_decimal = dot().and_then(uint, and_then_discard_both);
    let dot_with_opt_decimal = dot().and_then(opt(uint), and_then_discard_both);
    let decimal = uint.and_then(dot_with_opt_decimal, and_then_discard_both);
    let exp = (tag('e').or_else(tag('E')))
        .and_then(opt(sign), and_then_discard_both)
        .and_then(uint, and_then_discard_both);

    opt(sign)
        .and_then(dot_decimal.or_else(decimal), and_then_discard_both)
        .and_then(opt(exp), and_then_discard_both)(input)
}

fn sign(input: &str) -> ParserResult<()> {
    or(tag('-'), tag('+'), discard)(input)
}

fn uint(input: &str) -> ParserResult<()> {
    many(ascii_digit.map(discard), discard_both)(input)
}

fn dot() -> impl FnMut(&str) -> ParserResult<()> {
    tag('.')
}

fn main() {
    let input = ".1e";
    println!(
        "'{input}' {} a rationel number",
        if rationel(input).map(|buf| buf.0.is_empty()).unwrap_or(false) {
            "is"
        } else {
            "isn't"
        }
    );
}
