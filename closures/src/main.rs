#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    let a = |a: i32| a + 1;
    println!("a = {}", a(3));

    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    println!("b = {}", b(3));

    let gen = |x| println!("{}", x);
    gen(3);
    // gen(true); // error: mismatched types
}
