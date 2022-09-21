use std::ops::Add;

#[derive(Debug)]
enum Weight {
    Grams(i32),
    Kilos(i32),
}

impl Weight {
    fn in_grams(&self) -> i32 {
        match self {
            Self::Grams(v) => *v,
            Self::Kilos(v) => v * 1000,
        }
    }
}

impl Add for Weight {
    type Output = Weight;

    fn add(self, rhs: Self) -> Self::Output {
        Weight::Grams(self.in_grams() + rhs.in_grams())
    }
}

fn main() {
    println!("newtype");
    let x = Weight::Grams(100);
    let y = Weight::Kilos(1);
    println!("sum: {:?}", x + y);
}
