#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("{} barks", self.species)
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark());
}

fn main() {
    let dog = Dog { species: "Dog" };
    let cat = Cat { color: "Black" };
    bark_it(dog);
    // bark_it(cat); // error[E0277]: the trait bound `Cat: Bark` is not satisfied
}
