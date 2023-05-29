#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    // for i in 1..6 {
    //     say_hi();
    // }

    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}

// fn say_hi() {
//     println!("Hi!");
// }

fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello, {}!", name);
    greeting
}
