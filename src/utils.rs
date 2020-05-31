use nom::bytes::complete::take_while1;

pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(|c: char| c.is_whitespace())(s)
}
