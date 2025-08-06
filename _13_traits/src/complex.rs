use std::ops::Add;

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.0);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);
    // c1, c2 已经被移动，所以下面这句无法编译
    // println!("{:?}", c1 + c2);
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}
