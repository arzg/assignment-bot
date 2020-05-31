use nom::bytes::complete::{take_till, take_while1};

pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace)(s)
}

pub(crate) fn take_until_whitespace(s: &str) -> nom::IResult<&str, &str> {
    take_till(is_whitespace)(s)
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
