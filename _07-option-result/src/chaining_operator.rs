pub fn run() {
    // let r = read_file("assets/poem.txt");
    let r = read_file_v2("assets/poem.txt");
    // let r = read_file_verbose("assets/poem.txt");
    
    match r {
        Ok(content) => println!("{}", content),
        Err(e) => println!("{}", e),
    }
}

use std::fs::{self, File};
use std::io::{self, Error, Read};

fn read_file(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;

    let mut buf = String::new();
    let bytes: usize = file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn read_file_v2(file_path: &str) -> Result<String, Error> {
    let mut buf = String::new();
    let bytes: usize = File::open(file_path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_file_verbose(file_path: &str) -> Result<String, Error> {
    let r = File::open(file_path);

    let mut buf = String::new();

    // NOTE: read_to_string() returns the number of bytes.
    let r = match r {
        Ok(mut file) => file.read_to_string(&mut buf),
        Err(e) => return Err(e),
    };

    match r {
        Ok(_) => return Ok(buf),
        Err(e) => return Err(e)
    }
}