#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    let square = |a: i32| a * a;
    apply(square, 3);

    // calculate the sum of all the squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("sum = {}", sum);


    // with HOFs
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("sum2 = {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result: {}", f(a));
}
