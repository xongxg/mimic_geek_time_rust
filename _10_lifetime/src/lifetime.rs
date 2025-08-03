fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);
    println!("bigger one: {}", result);


    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

fn max<'a: 'b, 'b>(s1: &'a str, s2: &'b str) -> &'b str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn get_max(s: &str) -> &str {
    max(s, "Lindsey")
}
