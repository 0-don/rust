#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    println!("Hello, world!");
    println!("My name is {} and I'm {} years old", "John", 21);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "John", "dog", "cat");
    println!("{name} {surname}", surname = "Smith", name = "Alex");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);
    println!("array: {:?}", [1, 2, 3]);
}
