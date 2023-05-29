#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::Colors::Red;
use crate::Person::Name;

fn main() {
    let my_color = Red;
    println!("My color is {:?}", my_color);

    let my_color = Colors::Red;
    println!("My color is {:?}", my_color);

    let person = Name(String::from("John"));
    println!("Person: {:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
