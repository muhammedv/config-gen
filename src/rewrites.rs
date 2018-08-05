use regex::Regex;
use regex::Captures;
use std::borrow::Cow;
use url::Url;
use url::ParseError;

///
/// Replace the host name in a string
///
/// # Examples
///
/// ```rust
/// let bytes = "<a href=\"https://www.acme.com\">Home</a>";
/// let expected = "<a href=\"https://127.0.0.1:8000\">Home</a>";
/// assert_eq!(expected, replace_host(bytes, "www.acme.com", "127.0.0.1:8000"));
/// ```
///
pub fn replace_host<'a>(bytes: &'a str, host_to_replace: &'a str, target_host: &'a str, target_port: u16) -> Cow<'a, str> {
    let matcher = format!("https?://{}", host_to_replace);
    Regex::new(&matcher)
        .unwrap()
        .replace_all(bytes,
                     |item: &Captures|
                         modify_url(item, target_host, target_port).unwrap_or(String::from("")))
}

fn modify_url(caps: &Captures, host: &str, port: u16) -> Option<String> {
    let first_match = caps.iter().nth(0)?;
    let match_item = first_match?;

    if let Ok(mut url) = Url::parse(match_item.as_str()) {
        url.set_host(Some(host)).ok();
        url.set_port(Some(port)).ok();
        Some(url.to_string())
    } else {
        None
    }
}

#[test]
fn test_rewrites() {
    let bytes = "
    <a href=\"https://www.neomorganics.com\">Home</a>
    <a href=\"http://www.neomorganics.com\">Home</a>
    ";
    let expected = "
    <a href=\"https://127.0.0.1:8080/\">Home</a>
    <a href=\"http://127.0.0.1:8080/\">Home</a>
    ";
    let actual = replace_host(bytes, "www.neomorganics.com", "127.0.0.1", 8080);
    assert_eq!(actual, expected);
}
