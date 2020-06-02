use std::fmt;

pub(crate) struct Assignment {
    name: String,
    date: chrono::NaiveDate,
    notification: url::Url,
}

impl Assignment {
    pub fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, date) = crate::take_until_whitespace(s)?;

        let date = chrono::NaiveDate::parse_from_str(date, "%F")
            .map_err(|_| nom::Err::Error((s, nom::error::ErrorKind::Tag)))?;

        let (s, _) = crate::take_whitespace1(s)?;

        let (s, url) = crate::take_until_whitespace(s)?;

        let url =
            url::Url::parse(url).map_err(|_| nom::Err::Error((s, nom::error::ErrorKind::Tag)))?;

        let (s, _) = crate::take_whitespace1(s)?;

        Ok((
            "",
            Self {
                name: s.to_string(),
                date,
                notification: url,
            },
        ))
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "‘{}’, which is due on {}. Notification: {}",
            self.name,
            self.date.format("%d %B, %Y"),
            self.notification
        )
    }
}
