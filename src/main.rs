use calculator::{Equation, Error};
use std::io;
use std::io::Write;
mod calculator;

fn main() -> Result<(), Error>{
    loop{
        let mut input = String::new();
        print!("Enter expression (or q to quit): ");
        io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut input){
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed == "q" {
                    break;
                }

                let tokens = calculator::Equation::parse(input);
                if tokens.is_err(){
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let equation = calculator::Equation::equation(tokens?);
                if let Some(v) = calculator::Equation::evaluate(equation){
                    println!("Solution: {}", v);
                }
            },
            Err(error) => println!("Error: {}", error),
        }
    }
    Ok(())
}