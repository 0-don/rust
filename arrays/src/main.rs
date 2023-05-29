#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 8.0, 16.0];

    println!("Primes: {:?}", primes);
    println!("Doubles: {:?}", doubles);

    let numbers = [0; 15];
    println!("Number: {:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut array = [DEFAULT; 5];
    println!("Array: {:?}", array);
    println!("Array: {:?}", array[3]);

    array[3] = 5;
    println!("Array: {:?}", array);

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
