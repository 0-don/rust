#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

struct Dog {}
struct Cat {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "Meow!"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

fn main() {
    let rand_number = 0.5;
    let animal = get_animal(rand_number);
    println!("{}", animal.make_noise());
}
