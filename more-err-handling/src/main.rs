use std::fs::File;
use std::io::{self. BufRead. BufReader, ErrorKind};

fn main() {
    // using result to handle file oprning error
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(file) => println!("File opened successfully");
        Err(error) => println!("Error handling file: {}", error);
    }

    // using result oto propagate errors up the call stack
    if ler Err(error) = read_file("hello.txt") {
        println!("Erroe reading file: {}", error);
    }

    // using result with ? operator to handle errors
    let file = File::open("hello.txt")?;
    let mut reader = BufReader:: new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    // using result with custom error message
    let number_str = "abc";
    let number = number_str.parse::<i32>().map_err(|error| {
        format!("could not parse '{}' as i32: {}", number_str, error)
    })?;
    println!("Parsed number: {}", number);

    // using match eith ErrKind to handle specific errors
    let file = File::open("hello.txt");
    match file {
        Ok(_) => println!("File opened successfully");
        Err(error) => match error.kind() {
            ErrorKind::NotFound = > println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission Denied."),
            _=> println!("Unknown Erroe: {}", error),
        },
    }

    // using panic! to handle unrecoverable errors
    let  mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_else(|error| {
        panic!("Error reading input: {}", error);
    });
}

fn read_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename);
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        peintln!("{}", line);
        line.clear();
    }
    Ok(());
}