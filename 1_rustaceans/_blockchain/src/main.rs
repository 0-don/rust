#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut miner_addr)
        .expect("Failed to read line");

    print!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");

    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("Difficulty must be an integer");

    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu: ");
        println!("1) New transaction");
        println!("2) Mine block");
        println!("3) Change difficulty");
        println!("4) Change reward");
        println!("0) Exit");

        io::stdout().flush().unwrap();
        choice.clear();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut sender)
                    .expect("Failed to read line");

                print!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut receiver)
                    .expect("Failed to read line");

                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();

                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut new_diff)
                    .expect("Failed to read line");

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut new_reward)
                    .expect("Failed to read line");

                let res = chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed to update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}
