use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

fn main() {
    move_creates_issue()
}

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化后指向 name
    name_ptr: *const String,
    // PhantomPinned 占位符
    _marker: PhantomPinned,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_ptr: ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const _;
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
    }

    pub fn print_name(self: Pin<&Self>) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            // 在使用 ptr 是需要 unsafe
            // SAFETY: 因为数据不会移动，所以这里 name_ptr 是安全的
            unsafe { &*self.name_ptr },
        );
    }
}

fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:p})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) {
    println!("{:?} ({:p})", data, &data);
}

fn move_creates_issue() {
    let mut data = SelfReference::new("Tyr");
    let mut data = unsafe { Pin::new_unchecked(&mut data) };
    SelfReference::init(data.as_mut());

    data.as_ref().print_name();

    move_pinned(data.as_mut());
}
