fn main() {
    let g = Goose::new();
    let d = Duck::new(3);
    fly(g);
    fly(d);
}

trait Fly {
    fn fly(&self);
}

struct Goose;

impl Goose {
    pub fn new() -> Goose {
        Self
    }
}

impl Fly for Goose {
    fn fly(&self) {
        println!("Goose is flying");
    }
}

#[allow(dead_code)]
struct Duck {
    height: u8,
}

impl Duck {
    pub fn new(height: u8) -> Duck {
        Duck { height }
    }
}

impl Fly for Duck {
    fn fly(&self) {
        println!("Duck is flying");
    }
}

fn fly(fly: impl Fly) {
    fly.fly();
}

// impl Fly 作为返回值，需要有某个确定的类型，这样才能编译通过
// 这段代码无法提供确定的类型，所以出错
// fn select(name: &str) -> impl Fly {
//     match name {
//         "goose" => Goose::new() as Fly,
//         "duck" => Duck::new(3) as Fly,
//     }
// }
