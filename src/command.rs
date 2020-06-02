use nom::{branch::alt, bytes::complete::tag};

pub(crate) enum Command {
    Add(crate::Assignment),
    List,
    Help,
}

impl Command {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        alt((Self::new_add, Self::new_list, Self::new_help))(s)
    }

    fn new_add(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!add")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, assignment) = crate::Assignment::new(s)?;

        Ok((s, Self::Add(assignment)))
    }

    fn new_list(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!list")(s)?;

        Ok((s, Self::List))
    }

    fn new_help(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!help")(s)?;

        Ok((s, Self::Help))
    }
}
