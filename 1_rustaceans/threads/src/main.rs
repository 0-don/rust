#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use core::time::Duration;
use std::thread::{self, sleep};

fn main() {
    let mut threads = vec![];

    for i in 1..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(1000 * i));
            println!("Hello from thread number {}", i);
        });
        threads.push(th);
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Hello from main thread");
}
