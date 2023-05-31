use std::io;

fn main() {
    let mut input = String::new();

    // println! is a macro that prints text to the console
    println!("Say something");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input);
        }
        Err(e) => println!("Something went wrong: {}", e),
    }
}
