fn main() {
    let mut parts_of_string: Option<SplitStr> = None;
    // {
    let my_string = String::from("First line;Second line");
    parts_of_string = split(&my_string, ";");
    // }

    println!("{parts_of_string:?}");


}

#[derive(Debug)]
pub struct SplitStr<'a> {
    start: &'a str,
    end: &'a str,
}

pub fn split<'text, 'delim>(text: &'text str, delim: &'delim str) -> Option<SplitStr<'text>> {
    let (start, end) = text.split_once(delim)?;
    Some(SplitStr { start, end })
}
