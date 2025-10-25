// RUST DOES NOT HAVE EXCEPTIONS

use std::fs::File;
use std::io::{self, Read};

pub fn chapter_9_recoverable_errors() {
    {
        // Result type:
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    // return type of file::open is a Result
    let some_file = File::open("./test_file.txt");

    // so we can use let else syntax to extract file ptr from OK
    // let Ok(f) = File::open("./test_file.txt") else {
    //     println!("We got Err when opening file, No such exists");
    //     return;
    // };

    // or just simple match
    match some_file {
        Ok(f) => println!("Yay, file exists, we opened it"),
        Err(error) => {
            println!("there was an error while opening file: '{error}'")
        }
    }

    println!("############# UNWRAP AND EXPECT ##################");
    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well

    // If the Result value is Ok, UNWRAP RETURNS THE VALUE INSIDE the Ok.
    // If the Result is Err, unwrap will call the panic! macro
    //
    // let some_file = File::open("/.test_file.txt").unwrap();

    // the expect method lets us also choose the panic! error message.
    //
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // instead of panicking we return Result and propagate Err
    // We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately
    fn read_username_from_file() -> Result<String, io::Error> 
    {
        let mut file = match File::open("hello.txt") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // !!!!!!!!!!!!!!
    // !!!!!!!!!!!!!!
    // Instead of writing this shole function with matches and propagating error
    // in this VERBOSE way, Rust has a '?' operator that does exactly the same
    // BUT MUCH MUCH MORE CONCISE
    // !!!!!!!!!!!!!!
    // !!!!!!!!!!!!!!

    fn read_from_file_concise() -> Result<String, io::Error> 
    {
        let mut file = File::open("hello.txt")?; // ? operator does our match

        let mut username = String::new();

        file.read_to_string(&mut username)?;

        Ok(username)
    }


    // error values that have the ? operator called on them go through the from function which is used to convert values from one type into another

    // When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

    // We can chain these methods to make even shorter function

    fn read_from_file_shortest() -> Result<String, io::Error>
    {
        let mut username = String::new();

        File::open("test.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
}

fn chapter_9_unrecoverable_errors() {
    // unrecoverable errors are done by using panic! macro
    panic!("Programme panicked and ended execution");
}
