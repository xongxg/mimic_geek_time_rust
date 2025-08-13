use std::f32::consts::PI;
use std::ops::Mul;

fn main() {
    let c = curry(5);
    println!("c: {:?}", c(2));

    let adder = curry(PI);
    println!("pi multiply 4^2 is: {}", adder(4. * 4.));
}

fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
}
