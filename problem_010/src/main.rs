use common::input;
use common::math::IsPrime;

fn main() {
    println!("[INPUT] calculate primes until:");
    let limit = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    let mut sum = 0;

    for n in 2..limit {
        if n.is_prime() {
            sum += n;
        }
    }

    println!("[SOLUTION] sum of primes until {}: {}", limit, sum);
}
