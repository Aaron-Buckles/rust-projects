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

    pub fn add(mut a: Self, mut b: Self) -> Self {
        Self::common_denominator(&mut a, &mut b);
        let mut result = Self::new(a.numerator + b.numerator, a.denominator);
        result.simplify();
        result
    }

    fn common_denominator(a: &mut Self, b: &mut Self) {
        if a.denominator != b.denominator {
            a.numerator *= b.denominator;
            b.numerator *= a.denominator;

            let common_denominator = a.denominator * b.denominator;
            a.denominator = common_denominator;
            b.denominator = common_denominator;
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn main() {
    let f1 = Fraction::new(1, 3);
    let f2 = Fraction::new(1, 6);
    let result = Fraction::add(f1, f2);
    // dbg!(result);
    println!("{} + {} = {}", f1, f2, result);
}
