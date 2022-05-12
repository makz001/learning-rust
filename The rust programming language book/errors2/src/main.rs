use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
}

// A function that returns errors to the calling code using match ::

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A function that returns errors to the calling code using the ? operator ::

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chaining method calls after the ? operator ::

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Using fs::read_to_string instead of opening and then reading the file ::

fn read_username_from_file5() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
