use nom::{branch::alt, bytes::complete::tag};

pub(crate) enum Command {
    Add(crate::Assignment),
    List,
    Help,
    Delete(uuid::Uuid),
    Ping,
}

impl Command {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        alt((
            Self::new_add,
            Self::new_delete,
            Self::new_list,
            Self::new_help,
            Self::new_ping,
        ))(s)
    }

    fn new_add(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!add")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, assignment) = crate::Assignment::new(s)?;

        Ok((s, Self::Add(assignment)))
    }

    fn new_delete(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!delete")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let uuid = uuid::Uuid::parse_str(s)
            .map_err(|_| nom::Err::Error((s, nom::error::ErrorKind::Tag)))?;

        Ok(("", Self::Delete(uuid)))
    }

    fn new_list(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!list")(s)?;

        Ok((s, Self::List))
    }

    fn new_help(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!help")(s)?;

        Ok((s, Self::Help))
    }

    fn new_ping(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("!ping")(s)?;

        Ok((s, Self::Ping))
    }
}
