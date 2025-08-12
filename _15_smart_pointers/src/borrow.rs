fn main() {
    let s = "hello world!".to_owned();
    // 这里必须声明类型，因为 String 有多个 Borrow<T> 实现
    // 借用为 &String
    // let s1: &String = s.borrow();
    // let s2: &str = s.borrow();
}
