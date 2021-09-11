use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Neg, Mul, Div};
// use text_io::scan;

struct ParseFractionError;

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

    pub fn reciprocal(&self) -> Self {
        Fraction::new(self.denominator, self.numerator)
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

impl FromStr for Fraction {
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(i) = s.find('/') {
            let numerator = match s[..i].parse() {
                Ok(n) => n,
                Err(_) => return Err(ParseFractionError),
            };
            let denominator = match s[i+1..].parse() {
                Ok(d) => d,
                Err(_) => return Err(ParseFractionError),
            };

            return Ok(Fraction::new(numerator, denominator))
        }
        Err(ParseFractionError)
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

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self * other.reciprocal()
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

    let f16 = Fraction::new(1, 2);
    let f17 = Fraction::new(1, 4);
    let mut f18 = f16 / f17;
    f18.simplify();
    println!("{} / {} = {}", f16, f17, f18);

    let f19_str = "8/4";
    if let Ok(mut f19) = Fraction::from_str(f19_str) {
        f19.simplify();
        println!("\"{}\" simplifies to {}", f19_str, f19);
    } else {
        println!("Could not parse {}", f19_str);
    }
}
