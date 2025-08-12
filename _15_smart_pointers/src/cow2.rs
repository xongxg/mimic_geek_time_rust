use serde::Deserialize;
use std::borrow::Cow;

fn main() {
    let input = r#"{ "name": "Tyr", "age": 18 }"#;
    let user = serde_json::from_str::<User>(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("borrowed: {}", x),
        Cow::Owned(x) => println!("owned: {}", x),
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}
