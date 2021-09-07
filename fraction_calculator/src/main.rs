use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Neg, Mul};

// Another option would be the Binary GCD algorithm (https://en.wikipedia.org/wiki/Binary_GCD_algorithm)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    // Convert a and b to positive integers
    if a < 0 {
        a *= -1;
    }
    if b < 0 {
        b *= -1;
    }

    // Calculate the GCD
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
    numerator: i32,
    denominator: i32,
}

// TODO: Support negative fractions (need to use i32)
impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn simplify(&mut self) {
        let gcd = gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;

        if self.denominator < 0 {
            self.numerator *= -1;
            self.denominator *= -1;
        }
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
        self + -other
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.numerator, self.denominator)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
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

    let f8 = Fraction::new(1, 4);
    let f9 = Fraction::new(5, 4);
    let mut f10 = f8 - f9;
    f10.simplify();
    println!("{} - {} = {}", f8, f9, f10);

    let f11 = Fraction::new(-4, -8);
    let mut f12 = f11;
    f12.simplify();
    println!("{} = {}", f11, f12);

    let f13 = Fraction::new(2, 3);
    let f14 = Fraction::new(1,3);
    let mut f15 = f13 * f14;
    f15.simplify();
    println!("{} * {} = {}", f13, f14, f15);
}
