use regex::Regex;
use std::str::FromStr;

fn main() {
    println!("result: {:?}", u8::parse("255 hello world"));
}

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+(.[0-9]+)?").unwrap();
        let d = || Default::default();
        if let Some(m) = re.captures(s) {
            m.get(0).map_or(d(), |m| m.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}


#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("255 hello world"), 255);
    assert_eq!(u32::parse("123.45abcd"), 0);
    assert_eq!(f64::parse("123.45abcd").to_string(), "123.45");
    assert_eq!(f64::parse("abcd").to_string(), "0");
}