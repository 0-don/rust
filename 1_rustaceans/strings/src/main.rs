#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    let dog = String::new();
    let mut dog = String::from("Rusty");
    println!("{}", dog);

    let owner = format!("Hi I'm {} the owner of {}", "John", dog);
    println!("{}", owner);

    println!("{}", dog.len());

    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);
}
