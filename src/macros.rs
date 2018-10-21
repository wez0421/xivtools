use failure::Error;
use regex::Regex;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Action {
    pub name: String,
    pub wait: u64,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action ( action: {}, wait: {})", self.name, self.wait)
    }
}

fn parse_buffer(buffer: &str) -> Vec<Action> {
    let mut parsed_macros = vec![];
    buffer
        .trim()
        .lines()
        .for_each(|line| parsed_macros.push(parse_line(line.trim()).unwrap()));

    parsed_macros
}

pub fn parse_file(macros_file: PathBuf) -> Result<Vec<Action>, Error> {
    let buffer = fs::read_to_string(macros_file)?;
    Ok(parse_buffer(&buffer))
}

// Extract the action and wait times for a given line in a macros. Returns a
// String in the event of an error indicating a malformed macros.
pub fn parse_line(line: &str) -> Result<Action, String> {
    let re = Regex::new(r#"/ac ["]?([a-zA-Z' ]+[a-zA-Z])["]?(?: <wait.([0-9])>)?"#)
        .expect("error compiling regex");
    let values = re
        .captures(line)
        .ok_or_else(|| format!("Unable to parse line: {}", line))?;
    let action = values.get(1).map_or("", |m| m.as_str());
    let wait = match values.get(2) {
        Some(x) => x
            .as_str()
            .parse::<u64>()
            .map_err(|_| format!("failed to parse as number: {}", x.as_str()))?,
        None => 3,
    };

    Ok(Action {
        name: action.to_string(),
        wait,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macros_single_unqoted_no_wait() {
        // single word, unquoted, with no wait
        let entry = parse_line(r#"/ac Innovation"#).unwrap();
        assert_eq!(entry.name, "Innovation");
        assert_eq!(entry.wait, 3);
    }

    #[test]
    fn macros_single_qoted_no_wait() {
        // single word, quoted, with no wait
        let entry = parse_line(r#"/ac "Innovation""#).unwrap();
        assert_eq!(entry.name, "Innovation");
        assert_eq!(entry.wait, 3);
    }

    #[test]
    fn macros_single_unqoted_with_wait() {
        // single word, unquoted, with a wait
        let entry = parse_line(r#"/ac Innovation <wait.2>"#).unwrap();
        assert_eq!(entry.name, "Innovation");
        assert_eq!(entry.wait, 2);
    }

    #[test]
    fn macros_single_quoted_with_wait() {
        // single word, quoted, with a wait
        let entry = parse_line(r#"/ac "Innovation" <wait.2>"#).unwrap();
        assert_eq!(entry.name, "Innovation");
        assert_eq!(entry.wait, 2);
    }

    #[test]
    fn macros_double_quoted_no_wait() {
        // two words, quoted, with no wait
        let entry = parse_line(r#"/ac "Byregot's Blessing""#).unwrap();
        assert_eq!(entry.name, "Byregot's Blessing");
        assert_eq!(entry.wait, 3);
    }

    #[test]
    fn macros_double_quoted_with_wait() {
        // two words, quoted, with a wait
        let entry = parse_line(r#"/ac "Byregot's Blessing" <wait.3>"#).unwrap();
        assert_eq!(entry.name, "Byregot's Blessing");
        assert_eq!(entry.wait, 3);
    }

    #[test]
    fn macros_empty() {
        let result = parse_line(r#""#);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn macros_buffer() {
        let test_macros = r#"
        /ac "Comfort Zone" <wait.3>
        /ac "Inner Quiet" <wait.2>
        /ac "Great Strides" <wait.2>
        /ac "Manipulation II" <wait.3>
        /ac "Byregot's Blessing" <wait.3>
        /ac "Careful Synthesis III" <wait.3>"#;

        let actual = parse_buffer(test_macros);
        assert_eq!(validate_test_entries(actual), true);
    }

    #[test]
    fn macros_file() {
        let actual = parse_file(PathBuf::from("src/test_macro"));
        assert_eq!(validate_test_entries(actual.unwrap()), true);
    }

    fn validate_test_entries(actual: Vec<Action>) -> bool {
        let expected = [
            Action {
                name: "Comfort Zone".to_string(),
                wait: 3,
            },
            Action {
                name: "Inner Quiet".to_string(),
                wait: 2,
            },
            Action {
                name: "Great Strides".to_string(),
                wait: 2,
            },
            Action {
                name: "Manipulation II".to_string(),
                wait: 3,
            },
            Action {
                name: "Byregot's Blessing".to_string(),
                wait: 3,
            },
            Action {
                name: "Careful Synthesis III".to_string(),
                wait: 3,
            },
        ];

        (actual == expected)
    }
}
