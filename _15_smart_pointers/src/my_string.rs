use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

fn main() {
    let len1 = size_of::<MyString>();
    let len2 = size_of::<MiniString>();
    println!("len1 = {}, len2 = {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
    println!("s1 = {}, s2 = {}", s1, s2);

    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );
}

const MINI_STRING_MAX_LEN: usize = 30;

pub struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 我们在拷贝内容时一定要要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Standard(ref s) => s.deref(),
            MyString::Inline(ref s) => s.deref(),
        }
    }
}

impl<T> From<T> for MyString
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref().len() > MINI_STRING_MAX_LEN {
            true => MyString::Standard(value.as_ref().to_string()),
            _ => MyString::Inline(MiniString::new(value)),
        }
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Standard(ref mut v) => v.push_str(s),
            MyString::Inline(ref mut v) => {
                let len = v.len();
                let len1 = s.len();
                if len + len1 > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    let total = len + len1;
                    v.data[len..len + len1].copy_from_slice(s.as_bytes());
                    v.len = total as u8;
                }
            }
        }
    }
}
