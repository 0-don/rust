#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use core::time::Duration;
use std::{
    sync::mpsc,
    thread::{self, sleep},
};

const NUM_THEADS: usize = 20;

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("setting timer {}", d);
        sleep(Duration::from_secs(d as u64));
        println!("sending {}", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     tx.send(42).unwrap();
    // });

    // println!("received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    for d in 0..NUM_THEADS {
        start_thread(d, tx.clone());
    }

    for j in rx.iter().take(NUM_THEADS) {
        println!("received {}", j);
    }
}
