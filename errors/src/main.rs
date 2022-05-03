use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Calling panic macro ::
    // panic!("crash and burn!");
    
    // Attempting to access an element beyond the end of a vector, which will
    // cause a call to panic! ::
    // let v = vec![1, 2, 3];
    // v[99];
    
    // Using a match expression to handle the result variants that might be returned ::
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Handling different kinds of errors in different ways ::

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },  
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Handling errors with unwrap() ::
    
    // let f = File::open("hello.txt").unwrap();

    // Handling errors with expect() ::

    //let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
