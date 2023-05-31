#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let mut file = File::create("src/hello.txt").expect("Could not create file.");

    let mut file = OpenOptions::new()
        .append(true)
        .open("src/hello.txt")
        .expect("Could not open file.");
    file.write_all(b"Hello, world!\n")
        .expect("Could not write to file.");

    let mut file = File::open("src/hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    remove_file("src/hello.txt").expect("Could not remove file.");
}
