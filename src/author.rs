use std::error;
use std::fmt;
use std::str::FromStr;

use regex::Regex;

use git_config_format::GitConfigFormat;
use patch_format::PatchFormat;

#[derive(Debug, PartialEq)]
pub struct Author {
    pub alias: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Parse Error")
    }
}

impl error::Error for ParseError {}

impl<'a> FromStr for Author {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Author, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\S+)\s*\|\s*(.+?)\s*\|\s*(\S+)\s*$").unwrap();
        }

        match RE.captures(value) {
            Some(ref captures) if captures.len() == 4 => Ok(Author {
                alias: captures[1].into(),
                name: captures[2].into(),
                email: captures[3].into(),
            }),
            _ => Err(ParseError),
        }
    }
}

impl PatchFormat for Author {
    fn format(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

impl GitConfigFormat for Author {
    fn format(&self) -> String {
        format!("{} | {} | {}", self.alias, self.name, self.email)
    }
}

impl fmt::Display for Author {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}] {} <{}>", self.alias, self.name, self.email)
    }
}

#[cfg(test)]
mod tests {
    use super::Author;
    use super::ParseError;
    use super::PatchFormat;

    #[test]
    fn test_format_for_patch() {
        let author = Author {
            alias: "doggo".into(),
            name: "Really Good Doggo".into(),
            email: "doggo113@email.com".into(),
        };

        assert_eq!(
            &author.format()[..],
            "Really Good Doggo <doggo113@email.com>"
        );
    }

    #[test]
    fn test_parse_alias() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author: Author = line.parse().unwrap();
        assert_eq!(author.alias, "doggo");
    }

    #[test]
    fn test_parse_name() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author: Author = line.parse().unwrap();
        assert_eq!(author.name, "Really Good Doggo");
    }

    #[test]
    fn test_parse_email() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author: Author = line.parse().unwrap();
        assert_eq!(author.email, "doggo113@email.co.uk");
    }

    #[test]
    fn test_parse_unexpected_format() {
        let line = "doggo | ";
        assert_eq!(line.parse::<Author>(), Err(ParseError));
    }
}
