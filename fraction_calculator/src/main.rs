use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::cmp::Ordering;

// Another option would be the Binary GCD algorithm (https://en.wikipedia.org/wiki/Binary_GCD_algorithm)
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while a != b {
        match a.cmp(&b) {
            Ordering::Less => b -= a,
            Ordering::Equal => break,
            Ordering::Greater => a -= b,
        }
    }
    a
}

#[derive(Debug, Clone, Copy)]
struct Fraction {
    numerator: u32,
    denominator: u32,
}

// TODO: Support negative fractions (need to use i32)
impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn simplify(&mut self) {
        let gcd = gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = if self.denominator == other.denominator {
            Fraction::new(self.numerator + other.numerator, self.denominator)
        } else {
            Fraction::new(
                self.numerator * other.denominator + other.numerator * self.denominator,
                self.denominator * other.denominator,
            )
        };
        result.simplify();
        result
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn main() {
    let f1 = Fraction::new(1, 4);
    let f2 = Fraction::new(2, 4);
    let f3 = f1.add(&f2);
    println!("{} + {} = {}", f1, f2, f3);

    let f4 = Fraction::new(5, 4);
    let f5 = Fraction::new(3, 4);
    let f6 = Fraction::new(8, 4);
    let f7 = f4.add(&f5).add(&f6);
    println!("{} + {} + {} = {}", f4, f5, f6, f7);
}
