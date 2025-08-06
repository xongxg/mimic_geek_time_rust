fn main() {
    let cat = Cat;
    println!("cat: {}", animal(cat));
}

trait Animal {
    fn name(&self) -> &'static str;
}

struct Cat;

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }
}

struct Dog;
impl Animal for Dog {
    fn name(&self) -> &'static str {
        "dog"
    }
}

fn animal(animal: impl Animal) -> &'static str {
    animal.name()
}
