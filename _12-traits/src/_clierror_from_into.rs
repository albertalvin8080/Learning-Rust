use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    IOError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IOError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

/*
The `?` Operator unwraps the returned value from the function, or returns
an error which is then wrapped into the Result<i32, CliError> and prevents
the execution of the rest of read_number().
*/
fn read_number(file_path: &str) -> Result<i32, CliError> {
    let content: String = fs::read_to_string(&file_path)?;
    let num: i32 = content.trim().parse()?;
    Ok(num)
}

pub fn run() {
    // let r: Result<i32, CliError> = read_number("./number.txt");
    let r: Result<i32, CliError> = read_number("./src/number.txt");

    if let Ok(n) = r {
        println!("{}", n);
    }

    if let Err(e) = r {
        println!("Error: {:?}", e);
    }
}
