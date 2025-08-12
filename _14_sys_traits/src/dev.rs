fn main() {
    let dev = Developer {
        name: "Try".into(),
        age: 18,
        lang: Language::Rust,
    };

    let dev1 = dev.clone();
    println!("dev: {:?}, addr of name: {:p}", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of name: {:p}", dev1, dev1.name.as_str());
}

#[derive(Debug, Clone, Copy)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Developer {
    name: String,
    age: u8,
    lang: Language,
}
