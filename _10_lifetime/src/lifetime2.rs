fn main() {
    let s1 = String::from("Lindsey");
    let result;
    {
        let s2 = String::from("Rosie");
        result = max(&s1, &s2);

        // println!("result: {}", result);
    }

    // println!("result: {}", result);
}

fn max<'a: 'b, 'b>(s1: &'a String, s2: &'b String) -> &'b String {
    if s1.len() > s2.len() { s1 } else { s2 }
}
