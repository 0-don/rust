#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 0.5, y: 3.2 };
    let p3 = p1 + p2;
    println!("p3 = {:?}", p3);
}
