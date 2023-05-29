use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();

    println!("Random number: {}", i);

    println!("bounded int: {}", rng.gen_range(0..100));
    println!("bounded float: {}", rng.gen_range(0.0..10.0));

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("Random string: {}", rand_string);
}
