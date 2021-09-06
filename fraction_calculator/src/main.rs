use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::cmp::Ordering;
use std::ops::{Add, Sub};

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

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.denominator == other.denominator {
            Fraction::new(self.numerator + other.numerator, self.denominator)
        } else {
            Fraction::new(
                self.numerator * other.denominator + other.numerator * self.denominator,
                self.denominator * other.denominator,
            )
        }
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.denominator == other.denominator {
            Fraction::new(self.numerator - other.numerator, self.denominator)
        } else {
            Fraction::new(
                self.numerator * other.denominator - other.numerator * self.denominator,
                self.denominator * other.denominator
            )
        }
    }
}

fn main() {
    let f1 = Fraction::new(1, 4);
    let f2 = Fraction::new(2, 4);
    let mut f3 = f1 + f2;
    f3.simplify();
    println!("{} + {} = {}", f1, f2, f3);

    let f4 = Fraction::new(5, 4);
    let f5 = Fraction::new(3, 4);
    let f6 = Fraction::new(8, 4);
    let mut f7 = f4 + f5 + f6;
    f7.simplify();
    println!("{} + {} + {} = {}", f4, f5, f6, f7);

    let mut f8 = f2 - f1;
    f8.simplify();
    println!("{} - {} = {}", f2, f1, f8);
}
