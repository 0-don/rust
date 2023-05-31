#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;

fn main() {
    // let v = vec![1, 2, 3];
    // v[10];
    // panic!("crash and burn");

    let f = File::open("hello.txt");
    match f {
        Ok(f) => {
            println!("File opened successfully. {:?}", f);
        }
        Err(e) => {
            println!("Failed to open file. {:?}", e);
        }
    }
    println!("Continuing...");

    divide(Some(1));
    // divide(Some(0));
    divide(None);
}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Divide by zero error!"),
        Some(x) => println!("result is {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
    }
}
