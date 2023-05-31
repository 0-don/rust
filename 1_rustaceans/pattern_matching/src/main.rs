#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    for i in 0..15 {
        println!("{}: I have {}", i, get_oranges(i));
    }

    let point = (0, 0);
    let point = (3, 0);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x-axis, x = {}", x),
        (0, y) => println!("y-axis, y = {}", y),
        (x, y) => println!("({}, {})", x, y),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    match amount {
        0 => "no oranges",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "even number",
        _ => "a bunch",
    }
}
