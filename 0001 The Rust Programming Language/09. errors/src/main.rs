use std::fs;
use std::fs::File;
use std::io;
// use std::io::ErrorKind;
use std::io::Read;

fn main() {
    {
        // let v = vec![1, 2, 3];

        // v[99];
    }
    {
        let _file_io = File::open("hello.txt");

        // let _f = match _file_io {
        //     Ok(file) => file,
        //     Err(e) => match e.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {:?}", e),
        //         },
        //         other_error => {
        //             panic!("Problem opening the file: {:?}", other_error)
        //         }
        //     },
        // };

        // vs.

        // let _f = File::open("hello.txt").unwrap_or_else(|error| {
        //     if error.kind() == ErrorKind::NotFound {
        //         File::create("hello.txt").unwrap_or_else(|error| {
        //             panic!("Problem creating the file: {:?}", error);
        //         })
        //     } else {
        //         panic!("Problem opening the file: {:?}", error);
        //     }
        // });
    }
    {
        // File::open("hello.txt").unwrap(); // standard error message

        // // vs.

        // File::open("hello.txt").expect("Custom error message.");
    }
    {
        println!("{:?}", read_username_from_file());
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

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

// vs.

fn _read_username_from_file1() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// vs.

fn _read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
