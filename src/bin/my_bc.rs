use std::io::BufRead;

use my_bc::my_bc;

fn main() {
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(expression) => {
                match expression.as_str() {
                    "quit" => break,
                    _ => {
                        match my_bc::eval(&expression) {
                            Ok(result) => println!("{}", result),
                            Err(err) => println!("error: {}", err),
                        }
                    },
                }
            },
            Err(err) => println!("error: {}", err),
        }
    }
}
