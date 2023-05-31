#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let primses: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5, 7, 11];
    println!("Primes: {:?}", primes);

    primes.push(13);
    println!("Primes: {:?}", primes);
    primes.remove(2);
    println!("Primes: {:?}", primes);

    let mut numbers = vec![2; 10];
    println!("Numbers: {:?}", numbers);

    const DEFAULT: bool = true;
    let mut values = vec![DEFAULT; 5];
    println!("Values: {:?}", values);

    numbers[5] = 8;
    println!("Numbers: {:?}", numbers);

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
