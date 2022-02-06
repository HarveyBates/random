use std::fs::File;
use std::io::{self, Read, ErrorKind};

// Enum result has two types as below
//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("Hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Same funtionality as above but uses the ? operator
fn better_read_username_from_file() -> Result<String, io::Error> {
    // The ? operator means:
    // If a value is found the program will continue
    // If an error occurs the line will return from the function
    // To be handled by the caller
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chaining ? operator
fn even_better_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("Hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// ? can be used on Option<T> values as well
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn main() {
    // panic!("crash and burn"); // Exits with error
    
    let f = File::open("Hello.txt"); // Returns Result

    // If file is not found, create file with error handling
    // Basic method
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => {
                    println!("File not found creating file");
                    fc
                },
                Err(e) => panic!("Problem creating file: {:?}", e),
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // Method two using closures
    // Does the same thing but just cleaner using the unwrap_or_else method
    let f2 = File::open("Hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello2.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // Method three using unwrap only
    // Will return Ok() if file is found or panic! if else
    //let f3 = File::open("Hello3.txt").unwrap();
    

    // Method four using expect
    // Returns our error message if panic! occurs
    //let f4 = File::open("Hello4.txt").expect("Failed to open file Hello4.txt");


    // Propagating errors
    // Passing them back to the called for them to handle
    let username = match read_username_from_file() {
        Ok(u) => {
            println!("Username: {}", &u.trim());
            u
        },
        Err(e) => panic!("No username found"),
    };
    
    // Ignores errors
    if let Some(last_char) = last_char_of_first_line(&username) {
        println!("Last char: {}", last_char);
    }



}
