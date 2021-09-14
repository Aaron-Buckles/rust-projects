use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug)]
pub struct ParseFractionError;

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
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn clone_simplified(&self) -> Self {
        let mut f = self.clone();
        f.simplify();
        f
    }

    pub fn simplify(&mut self) {
        if self.is_zero() || self.is_undefined() {
            return;
        }

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

    pub fn is_same_as(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub fn is_undefined(&self) -> bool {
        self.denominator == 0
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

        if let Ok(num) = s.parse::<i32>() {
            return Ok(Fraction::from(num))
        }

        Err(ParseFractionError)
    }
}

impl From<i32> for Fraction {
    fn from(num: i32) -> Self {
        Fraction::new(num, 1)
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

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        if self.is_same_as(&other) {
            return true;
        }

        let f1 = self.clone_simplified();
        let f2 = other.clone_simplified();
        f1.is_same_as(&f2)
    }
}

impl PartialEq<i32> for Fraction {
    fn eq(&self, other: &i32) -> bool {
        *self == Fraction::from(*other)
    }
}

impl PartialEq<Fraction> for i32 {
    fn eq(&self, other: &Fraction) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use crate::fraction::Fraction;
    use std::str::FromStr;

    #[test]
    fn is_undefined() {
        assert!(Fraction::new(5, 0).is_undefined())
    }

    #[test]
    fn is_zero() {
        assert!(Fraction::new(0, 10).is_zero());
    }

    #[test]
    fn is_same_as() {
        let f1 = Fraction::new(123, 456);
        let f2 = Fraction::new(123, 456);
        assert!(f1.is_same_as(&f2));
    }

    #[test]
    fn is_not_same_as() {
        let f1 = Fraction::new(123, 456);
        let f2 = Fraction::new(456, 123);
        assert!(!f1.is_same_as(&f2));
    }

    #[test]
    fn reciprocal() {
        let f1 = Fraction::new(123, 456);
        let result = f1.clone().reciprocal();
        let expected = Fraction::new(456, 123);
        assert_eq!(result, expected);
    }

    #[test]
    fn equality() {
        let f1 = Fraction::new(6, 12);
        let f2 = Fraction::new(1, 2);
        assert_eq!(f1, f2);
    }

    #[test]
    fn equality_with_int() {
        let f1 = Fraction::new(3, 1);
        let expected = f1.numerator;
        assert_eq!(f1, expected);
    }

    #[test]
    fn equality_with_zero() {
        assert_eq!(Fraction::new(0, 1), 0);
    }

    #[test]
    fn simplify_positive() {
        let mut f1 = Fraction::new(8, 32);
        f1.simplify();
        let expected = Fraction::new(1, 4);
        assert_eq!(f1.numerator, expected.numerator);
        assert_eq!(f1.denominator, expected.denominator);
    }

    #[test]
    fn simplify_negative() {
        let mut f1 = Fraction::new(-3, 9);
        f1.simplify();
        let expected = Fraction::new(-1, 3);
        assert_eq!(f1.numerator, expected.numerator);
        assert_eq!(f1.denominator, expected.denominator);
    }

    #[test]
    fn simplify_zero() {
        let mut f1 = Fraction::new(0, 1);
        f1.simplify();
        assert_eq!(f1, 0);
    }

    #[test]
    fn simplify_double_negative() {
        let mut f1 = Fraction::new(-5, -25);
        f1.simplify();
        let expected = Fraction::new(1, 5);
        assert_eq!(f1.numerator, expected.numerator);
        assert_eq!(f1.denominator, expected.denominator);
    }

    #[test]
    fn addition_with_common_denominator() {
        let f1 = Fraction::new(1, 5);
        let f2 = Fraction::new(3, 5);
        let result = f1 + f2;
        let expected = Fraction::new(4, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_without_common_denominator() {
        let f1 = Fraction::new(1, 3);
        let f2 = Fraction::new(1, 2);
        let result = f1 + f2;
        let expected = Fraction::new(5, 6);
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_with_two_negatives() {
        let f1 = Fraction::new(-2, 3);
        let f2 = Fraction::new(-4, 8);
        let result = f1 + f2;
        let expected = Fraction::new(-7, 6);
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_with_one_negative_first() {
        let f1 = Fraction::new(-7, 14);
        let f2 = Fraction::new(1, 1);
        let result = f1 + f2;
        let expected = Fraction::new(1, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_with_one_negative_last() {
        let f1 = Fraction::new(3, 5);
        let f2 = Fraction::new(-2, 5);
        let result = f1 + f2;
        let expected = Fraction::new(1, 5);
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_of_three_fractions() {
        let f1 = Fraction::new(5, 4);
        let f2 = Fraction::new(3, 4);
        let f3 = Fraction::new(8, 4);
        let result = f1 + f2 + f3;
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn negative() {
        let f1 = Fraction::new(4, 3);
        let result = -f1;
        let expected = Fraction::new(-4, 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn subtraction() {
        let f1 = Fraction::new(6, 7);
        let f2 = Fraction::new(2, 7);
        let result = f1 - f2;
        let expected = Fraction::new(4, 7);
        assert_eq!(result, expected);
    }

    #[test]
    fn from_string() {
        let result = Fraction::from_str("6/18").unwrap();
        let expected = Fraction::new(1, 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn from_string_to_whole_number() {
        let result = Fraction::from_str("8/4").unwrap();
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication() {
        let f1 = Fraction::new(5, 8);
        let f2 = Fraction::new(6,12);
        let result = f1 * f2;
        let expected = Fraction::new(30, 96);
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_to_whole_number() {
        let f1 = Fraction::new(9, 3);
        let f2 = Fraction::new(1,3);
        let result = f1 * f2;
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn division() {
        let f1 = Fraction::new(1, 6);
        let f2 = Fraction::new(1, 3);
        let result = f1 / f2;
        let expected = Fraction::new(1, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn division_to_whole_number() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 4);
        let result = f1 / f2;
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn complex_equation1() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 4);
        let result = (f1 + f2 - f2) / f2 * -f1;
        let expected = -1;
        assert_eq!(result, expected);
    }
}