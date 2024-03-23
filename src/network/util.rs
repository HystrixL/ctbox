use regex::Regex;

/// dr1004\({}\)
/// r"(dr1004\()(?P<content>[\s\S]*?)(\))"
fn generate_regex(template: &str) -> String {
    let re = Regex::new(r"^(?P<head>.*?)(?P<_>\{})(?P<tail>.*?)$").unwrap();

    let head = re
        .captures(template)
        .and_then(|cap| cap.name("head").map(|head| head.as_str()))
        .unwrap();

    let tail = re
        .captures(template)
        .and_then(|cap| cap.name("tail").map(|tail| tail.as_str()))
        .unwrap();

    format!(r"({head})(?P<content>[\s\S]*?)({tail})")
}

pub fn fuck_cnu_api<'a>(source: &'a str, template: &'a str) -> &'a str {
    let re = Regex::new(&generate_regex(template)).unwrap();

    re.captures(source)
        .and_then(|cap| cap.name("content").map(|content| content.as_str()))
        .unwrap()
}

pub fn is_cnu() -> bool {
    if let Ok(response) = reqwest::blocking::get(super::ENTRANCE) {
        response.status() == 200
    } else {
        false
    }
}