#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());

    // let b = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    // println!("sum = {}", b.sum());
}
