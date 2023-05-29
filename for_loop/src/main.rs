#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "hamster", "bear"];

    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("I don't like chihuahuas");
            continue;
        }

        if pet == &"hamster" {
            println!("I don't like bears");
            break;
        }

        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square)
    }
}
