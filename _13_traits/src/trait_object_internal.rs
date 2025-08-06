use std::fmt::{Debug, Display};
use std::mem::transmute;

fn main() {
    let s = String::from("hello world!");
    let s1 = String::from("goodbye world!");
    // Display / Debug trait object for s
    let w1: &dyn Display = &s;
    let w2: &dyn Debug = &s;

    // Display / Debug trait object for s1
    let w3: &dyn Display = &s1;
    let w4: &dyn Debug = &s1;

    // 强行把 triat object 转换成两个地址 (usize, usize)
    // 这是不安全的，所以是 unsafe
    let (addr1, vtable1): (usize, usize) = unsafe { transmute(w1) };
    let (addr2, vtable2): (usize, usize) = unsafe { transmute(w2) };
    let (addr3, vtable3): (usize, usize) = unsafe { transmute(w3) };
    let (addr4, vtable4): (usize, usize) = unsafe { transmute(w4) };

    // s 和 s1 在栈上的地址，以及 main 在 TEXT 段的地址
    println!(
        "s: {:p}, s1: {:p}, main(): {:p}",
        &s, &s1, main as *const ()
    );
}
