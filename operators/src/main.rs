#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let a = 4 + 8;
    let b = 10 / 3;
    let c = 10 % 3;
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("{}", a >= b);
    println!("{}", a >= b && b >= c);
}
