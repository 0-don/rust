#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

static mut R: i32 = 0;

fn main() {
    {
        let a = 3;
        println!("a = {}", a);
    }
    //    println!("a = {}", a); // error: a is not defined in this scope

    unsafe {
        R = 3;
        println!("R = {}", R);
    }

    unsafe {
        println!("R = {}", R);
    }
    
}
