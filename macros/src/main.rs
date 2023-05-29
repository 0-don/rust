#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

macro_rules! my_macro {
    () => {
        println!("Check out my macro!")
    };
}

// macro_rules! name {
//     ($name: expr) => {
//         println!("Hello, {}", $name)
//     };
// }

macro_rules! name {
    ($($name: expr),*) => {
        $(println!("Hello, {}", $name);)*
    };
}

macro_rules! xy {
    (x => $e: expr) => {
        println!("X: {}", $e)
    };
    (y => $e: expr) => {
        println!("Y: {}", $e)
    };
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("You called {:?}()", stringify!($fn_name));
        }
    };
}

fn main() {
    my_macro!();
    name!("Rust", "Python", "C++", "Java");
    println!("{} {} {} {}", "a", "b", "c", "d");
    xy!(x => 10);
    xy!(y => 20);
    build_fn!(hello);
    hello();
}
