#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..11);

    if num > 5 {
        println!("{} is greater than 5", num);
    } else if num == 5 {
        println!("{} is equal to 5", num);
    } else {
        println!("{} is less than 5", num);
    }

    // if num >= 5 {
    //     println!("{} is greater than or equal to 5", num);
    // } else {
    //     println!("{} is less than 5", num);
    // }

    let res = if num >= 5 { true } else { false };
    println!("res = {}", res);
}
