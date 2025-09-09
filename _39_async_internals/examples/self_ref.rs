use std::ptr;

fn main() {
    let data = move_creates_issue();
    println!("data: {:?}", data);
    // 如果把下面这句注释掉，程序运行会直接 segment error
    data.print_name();
    print!("\\n");
    mem_swap_creates_issue();
}

#[derive(Debug)]
pub struct SelfReference {
    name: String,
    name_ref: *const String,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_ref: ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.name_ref = &self.name as *const String;
    }

    pub fn print_name(&self) {
        println!(
            "struct {:p} (name: {:p} name_ref: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ref,
            self.name,
            unsafe { &*self.name_ref }
        )
    }
}


fn move_it(data: SelfReference) -> SelfReference {
    data
}


fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("Tyr");
    data1.init();

    let mut data2 = SelfReference::new("Lindsey");
    data2.init();

    data1.print_name();
    data2.print_name();

    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn move_creates_issue() -> SelfReference {
    let mut data = SelfReference::new("Tyr");
    data.init();

    // 不 move，一切正常
    data.print_name();

    let data = move_it(data);

    // move 之后，name_ref 指向的位置是已经失效的地址
    // 只不过现在 move 前的地址还没被回收挪作它用
    data.print_name();
    data
}


