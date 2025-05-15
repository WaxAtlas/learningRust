use std::{fs::{self, File}, error::Error, io::{self, ErrorKind, Read}};

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // A shortcut for Propgating Errors: the ? Operator

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // Shorter

    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // Even shorter!!!

    // fs::read_to_string("hello.txt")

}

fn main() -> Result<(), Box<dyn Error>> {
    
    /* Unrecoverable errors with panic! */

    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    /* Recoverable errors with Result<T, E> */

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // Matching on Different Errors

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     },
    // };

    // Alternatives to Using match with Result<T, E>

    /*
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    */

    // Propogating Errors (see fn read_username_from_file())

    // Chaging main function signature to allow the use of ?
    
    let greeting_file = File::open("hello.txt")?;

    Ok(())

}
