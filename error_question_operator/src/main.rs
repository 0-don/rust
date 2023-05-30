#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use std::io::Read;
use std::{fs::File, io};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("src/hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => return Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    println!("{:?}", a);
}
