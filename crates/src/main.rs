// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::Rng;

mod archive;

fn main() {
    arc("The Matrix");

    let mut rng = rand::thread_rng();
    let a = rng.gen::<i32>();
    println!("Random number: {}", a);
}
