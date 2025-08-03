use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

fn main() {
    // 使用 ManuallyDrop 封装数据结构使其不进行自动 drop
    let mut s = ManuallyDrop::new(MyString::from("ManuallyDrop"));

    // ManuallyDrop 使用了 Deref trait 指向 T，所以可以当 MyString 使用，MyString 又可以当 String 用
    s.truncate(3);
    println!("s = {:?}", s);

    // 如果没有这句，s 不会在 scope 结束时被自动 drop（你可以注掉试一下）
    // 如果我们想让它可以自动 drop，可以用 into_inner
    let _ = ManuallyDrop::into_inner(s);
}

#[derive(Debug)]
pub struct MyString(String);

impl From<&str> for MyString {
    fn from(value: &str) -> Self {
        MyString(value.to_string())
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("Dropping string {}", self.0);
    }
}

impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
