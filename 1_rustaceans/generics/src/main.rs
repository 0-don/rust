#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    let p1: Point<i32> = Point { x: 10, y: 20 };
    let p2: Point<f64> = Point { x: 10.0, y: 20.0 };
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let c1 = Colors::Red(10);
    let c2 = Colors::Green(10.0);
    let c3 = Colors::Blue("Hello");
    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
    println!("c3: {:?}", c3);

    let p3: Point2<i32, f64> = Point2 { x: 10, y: 20.0 };
    println!("p3: {:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V,
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Green(T),
    Blue(T),
}


