#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let th = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
            println!("Hello from thread number {}", i);
        });
        threads.push(th);
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("{}", *c.lock().unwrap());
}
