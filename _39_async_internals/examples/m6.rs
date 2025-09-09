fn insert_value<'vec_lifetime, 'contents_lifetime>(
    my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>,
    value: &'contents_lifetime i32,
) {
    my_vec.push(value)
}

// fn insert_value<'one_lifetime>(
//     my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>,
//     value: &'one_lifetime i32,
// ) {
//     my_vec.push(value)
// }

fn main() {
    let mut my_vec = vec![];
    let val1 = 1;
    let val2 = 2;

    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);

    println!("{my_vec:?}");
}
