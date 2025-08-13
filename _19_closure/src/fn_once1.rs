fn main() {
    let name = String::from("Tyr");
    let c = move |greeting: String| (greeting, name);

    let res = c("greeting".to_string());
    println!("res: {:?}", res);
}
