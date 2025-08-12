fn main() {
    let v = Data {
        inner: vec![1, 2, 3, 4, 5],
    };

    process(v);
    let v = UnSizedData {
        inner: vec![1, 2, 3, 4, 5],
    };
    // process(v);
}

#[allow(dead_code)]
struct Data<T> {
    inner: T,
}

fn process<T: Sized>(_data: Data<T>) -> T {
    todo!()
}

#[allow(dead_code)]
struct UnSizedData<T: ?Sized> {
    inner: T,
}

// 无法编译通过，函数的参数必须是编译时大小确定的
// #[allow(dead_code)]
// fn process_unsized_data<T>(_data: UnSizedData<T>)
// where
//     T: ?Sized,
// {
//     todo!()
// }
