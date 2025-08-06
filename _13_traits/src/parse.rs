use regex::Regex;

fn main() {
    println!("result: {}", u8::parse("255 hello world"));
}

trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(m) = re.captures(s) {
            m.get(0).map_or(0, |m| m.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}


#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);
}