#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::rc::Rc;

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand: Rc<String>) -> Car {
        Car { brand }
    }
    fn drive(&self) {
        println!("{} is driving.", self.brand);
    }
}

fn main() {
    let brand = Rc::new(String::from("Toyota"));
    println!("{} has {} owners.", brand, Rc::strong_count(&brand));
    {
        let car1 = Car::new(brand.clone());

        car1.drive();
        println!("{} has {} owners.", brand, Rc::strong_count(&brand));
    }

    println!("{} has {} owners.", brand, Rc::strong_count(&brand));
}
