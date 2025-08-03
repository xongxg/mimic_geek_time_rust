use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    let key = "hello1";

    match map.get_mut(key) {
        Some(value) => do_something(value),
        None => {
            map.insert(key, "tyr"); // <--- 这里又获得了一个可变引用
        }
    }
}

fn do_something(_v: &mut &str) {
    todo!()
}
