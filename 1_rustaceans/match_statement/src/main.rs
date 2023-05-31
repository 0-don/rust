#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    print_choice(Suit::Heart);
    print_choice(Suit::Spade);
    print_choice(Suit::Diamond);
    print_choice(Suit::Club);

    country(44);
    country(46);
    country(7);
    country(1);
    country(999);
    country(1000);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown",
        _ => "invalid",
    };

    println!("Country is {}", country);
}

#[derive(Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Club => println!("\u{2663}"),
        Suit::Diamond => println!("\u{2666}"),
        Suit::Heart => println!("\u{2665}"),
        Suit::Spade => println!("\u{2660}"),
    }
}
