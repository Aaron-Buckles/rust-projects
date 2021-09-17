use std::io;
use fraction::Fraction;
use equation::Equation;
use std::io::Write;

mod fraction;
mod equation;

fn main() {
    println!("Welcome to Aaron's Fraction Calculator!");
    println!("Numbers include fractions and whole numbers (e.g. -3/4, 2, 17/3)");
    println!("Operators include +, -, *, and /");
    println!("Numbers and operators need to be separated by a space");
    println!("Example: 1/2 - 3/4 * 7");
    println!("Type 'exit' to quit the program");

    loop {
        print!("$ ");

        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => {
                println!("There was an error flushing stdout, try again");
                continue;
            },
        };

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("There was an error trying to read the input, try again");
                continue;
            },
        };
        let input = input.trim();

        if input == "exit" {
            break;
        }

        match Equation::eval(&input) {
            Ok(mut result) => {
                result.simplify();
                println!(">>> {}", result)
            },
            Err(e) => println!("{}", e),
        };
    }
}
