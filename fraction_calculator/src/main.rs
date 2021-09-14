use fraction::Fraction;
use equation::Equation;

mod fraction;
mod equation;

fn main() {
    // Equation::eval("3 + 4");
    Equation::eval("3 + 4/5 * 6/7 + 1/2 - 2 * 3");
}
