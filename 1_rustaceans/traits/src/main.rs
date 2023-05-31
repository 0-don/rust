#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello, I'm a developer")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> RustDev {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("Hello, I'm a Rust developer")
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> JavaDev {
        JavaDev { awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello(&self) {
        println!("Hello, I'm a Java developer")
    }
}

fn main() {
    let r = RustDev { awesome: true };
    let r = RustDev::new(true);
    let j = JavaDev { awesome: false };
    let j = JavaDev::new(false);

    r.say_hello();
    j.say_hello();
}
