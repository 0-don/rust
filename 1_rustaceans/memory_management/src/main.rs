#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    let i = 5;
    let j = i;

    println!("i = {}, j = {}", i, j);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // println!("v = {:?}", v);
    // println!("w = {:?}", w);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("v = {:?}", v);
        v
    };

    let v = foo(v);
    println!("v = {:?}", v);
}


