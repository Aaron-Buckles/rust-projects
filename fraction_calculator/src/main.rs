use fraction::Fraction;
use equation::Equation;

mod fraction;
mod equation;

fn main() {
    // Equation::eval("3 + 4");
    Equation::eval("1/2 - 2 * 7/8")
}
