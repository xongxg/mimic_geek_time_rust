fn main() {
    // let b = local_ref();
    // println!("b: {:p}", b);

    let mut data = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data {:?}", data);



}

fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    // &a

    todo!();
}
