use std::collections::HashMap;

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}

fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(pos) = s.find(delimiter) {
        let prefix = &s[..pos];
        let suffix = &s[pos + delimiter.len_utf8()..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
