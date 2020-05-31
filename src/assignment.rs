use std::fmt;

pub(crate) struct Assignment {
    name: String,
}

impl Assignment {
    pub fn new(s: &str) -> nom::IResult<&str, Self> {
        Ok((
            "",
            Self {
                name: s.to_string(),
            },
        ))
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
