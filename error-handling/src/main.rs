use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    // File I/O errors
    let file = File::open("nonexistent.txt");
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            println!("File contents: {}", contents);
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e)
        }
    }

    // parsing errors
    let number_str = "not_a_number";
    let number =  number_str.parse::<i32>();
    match number {
        Ok(n) => {
            println!("Parsed number: {}", n);
        }

        Err(e) => match e {
            ParseIntError { .. } => {
                eprintln!("Error parsing integer: {}", e);
            }

            _=> {
                eprintln!("unknown error: {}", e);
            }
        },
    }

    // I/O errors
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("You entered: {}", input.trim());

    Ok(());
}