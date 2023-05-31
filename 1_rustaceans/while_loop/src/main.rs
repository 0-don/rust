#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::process::Command;

fn main() {
    get_square(3478);
    get_cubes(4938);
}

fn get_square(limit: i64) {
    let mut x = 1;
    while x * x < limit {
        // let square = x * x;
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
    let _ = Command::new("pause").status();
}

fn get_cubes(limit: i64) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x > limit {
            break;
        }
    }
}
