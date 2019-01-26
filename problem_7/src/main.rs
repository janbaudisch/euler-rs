use common::{input, math};

fn main() {
    println!("[INPUT] n:");
    let input = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    println!("[SOLUTION] {}th prime: {}", input, math::nth_prime(10001));
}
