#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }
    // fn get_name<'l>(&'l self) -> &'l String {
    //     &self.name
    // }
}

fn main() {
    println!("{}", get_str());

    let p1 = Person {
        name: String::from("Alice"),
    };
    let d1 = Dog {
        name: String::from("Bob"),
        owner: &p1,
    };

    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person {
            name: String::from("Charlie"),
        };
        a = p1.get_name();
        println!("{}", a);
    }
}

fn get_str() -> &'static str {
    "Hello, world!"
}
