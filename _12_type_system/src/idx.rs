fn main() {
    let int = idx(42);
    let string = idx("Tyr");
    println!("{}, {}", int, string);
}

fn idx<T>(x: T) -> T {
    x
}
