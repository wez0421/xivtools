use failure::format_err;
use failure::Error;
use regex::Regex;

// Extract the action and wait times for a given line in a macro. Returns a
// String in the event of an error indicating a malformed macro.
pub fn parse_macro_line(line: &str) -> Result<(&str, u32), String> {
    let re = match Regex::new(r#"/ac ["]?([a-zA-Z' ]+[a-zA-Z])["]?(?: <wait.([0-9])>)?"#) {
        Ok(v) => v,
        Err(e) => panic!("error compiling regex: {}", e),
    };

    match re.captures(line) {
        Some(values) => {
            let action = values.get(1).map_or("", |m| m.as_str());
            let wait = values
                .get(2)
                .map_or(3, |m| m.as_str().parse::<u32>().unwrap());
            Ok((action, wait))
        }
        None => Err(format!("Unable to parse line: {}", line)),
    }
}

#[test]
fn test_macro_single_unqoted_no_wait() -> Result<(), String> {
    // single word, unquoted, with no wait
    let (action, wait) = parse_macro_line(r#"/ac Innovation"#)?;
    assert_eq!(action, "Innovation");
    assert_eq!(wait, 3);
    Ok(())
}

#[test]
fn test_macro_single_qoted_no_wait() {
    // single word, quoted, with no wait
    let (action, wait) = parse_macro_line(r#"/ac "Innovation""#).unwrap();
    assert_eq!(action, "Innovation");
    assert_eq!(wait, 3);
}

#[test]
fn test_macro_single_unqoted_with_wait() {
    // single word, unquoted, with a wait
    let (action, wait) = parse_macro_line(r#"/ac Innovation <wait.2>"#).unwrap();
    assert_eq!(action, "Innovation");
    assert_eq!(wait, 2);
}

#[test]
fn test_macro_single_quoted_with_wait() {
    // single word, quoted, with a wait
    let (action, wait) = parse_macro_line(r#"/ac "Innovation" <wait.2>"#).unwrap();
    assert_eq!(action, "Innovation");
    assert_eq!(wait, 2);
}

#[test]
fn test_macro_double_quoted_no_wait() {
    // two words, quoted, with no wait
    let (action, wait) = parse_macro_line(r#"/ac "Byregot's Blessing""#).unwrap();
    assert_eq!(action, "Byregot's Blessing");
    assert_eq!(wait, 3);
}

#[test]
fn test_macro_double_quoted_with_wait() {
    // two words, quoted, with a wait
    let (action, wait) = parse_macro_line(r#"/ac "Byregot's Blessing" <wait.3>"#).unwrap();
    assert_eq!(action, "Byregot's Blessing");
    assert_eq!(wait, 3);
}

#[test]
fn test_macro_whitespace() {
    let result = parse_macro_line(r#"                     "#);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_macro_empty() {
    let result = parse_macro_line(r#""#);
    assert_eq!(result.is_err(), true);
}
