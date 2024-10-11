/*
 * Rust groups errors into two categories: recoverable and unrecoverable errors.
 * Recoverable      : we just want to report. e.g a file is not found
 * Unrecoverable    : We want to stop the program when we encounter this error.
 * Rust does not have exceptions, it has the type Result<T,E> for recoverable
 * errors and the panic! that stops execution when it encounters
 * unrecoverable errors.
 *
 * When panic! is called, Rust unwinds. It walks back up the stack and cleans
 * up data from each function it encounters. It is a lot of work. Rust allows
 * you to choose the alternative which is immediately aborting which ends the
 * program without cleaning up.
 *
 * Add this to Cargo.tom
 * [profile.release]
 * panic = 'abort'
 *
 */

#![allow(unused_variables, dead_code)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash an burn");

    // this code will also panic because we're trying to access an index that
    // does not exist
    /*
     * let v = vec![1, 2, 3];
     * v[99];
     */

    // recoverable errors
    // let's try to open a file

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening the file {error:?}"),
    // };

    // we want to check different error kinds
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // we can also use closures to shorten this logic
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating file: {error:?}");
            })
        } else {
            panic!("Problem creating file : {error:?}");
        }
    });

    // if we want to call panic by default, we can just use unwrap
    // let some_file = File::open("somefile.txt").unwrap();

    // if we want to choose the panic message, we can use except
    // let some_file = File::open("somefile.txt").expect("somefile.txt should be included in this project");

    let username = match read_username_from_file() {
        Ok(u) => u,
        Err(e) => panic!("failed to read username because : {e:?}"),
    };
    println!("Read the username as: {username}");

    let username = match read_username_from_file_short() {
        Ok(u) => u,
        Err(e) => panic!("failed to read username because : {e:?}"),
    };
    println!("Read the username as: {username}");

    let username = read_username_from_file().unwrap();

    println!("Read the username as: {username}");
}

// this function returns either a result or an error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; // if the file exists, its now in username_file, otherwise, username_file
       // contains an error
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // when we use ?, we're
                                                      // basically doing match.
                                                      // if a value exists,
                                                      // its returned.
                                                      // if not, an error is.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
