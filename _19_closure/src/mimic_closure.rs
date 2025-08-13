fn main() {
    let name = "Tyr".into();
    // 模拟变量捕捉
    let c = ClosureOnce {
        captured: (name,),
        func: greeting_code1,
    };

    // 模拟闭包调用，这里和 FnOnce 不完全一样，传入的是一个 tuple 来匹配 Args 参数
    println!("{:?}", c.call_once(("hola".into(),)));

    // 更复杂一些的复杂的闭包
    let c1 = ClosureOnce {
        captured: ("Tyr".into(), 18),
        func: greeting_code2,
    };

    println!("{:?}", c1.call_once(("hola".into(), "hallo".into())));
    // println!("{:?}", c1.call_once(("hola".into(), "hallo".into())));
}

struct ClosureOnce<Captured, Args, Output> {
    captured: Captured,
    func: fn(Args, Captured) -> Output,
}

impl<Captured, Args, Output> ClosureOnce<Captured, Args, Output> {
    fn call_once(self, args: Args) -> Output {
        (self.func)(args, self.captured)
    }
}

// 类似 greeting 闭包的函数体
fn greeting_code1(args: (String,), captured: (String,)) -> (String, String) {
    (args.0, captured.0)
}

fn greeting_code2(args: (String, String), captured: (String, u8)) -> (String, String, String, u8) {
    (args.0, args.1, captured.0, captured.1)
}

