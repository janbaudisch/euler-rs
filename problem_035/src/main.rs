use common::input;
use common::math::IsPrime;

fn main() {
    println!("[INPUT] calculate primes until:");
    let limit = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    let mut count = 0;

    for n in 2..limit {
        if n.is_prime() {
            let mut digits = n.to_string().chars().collect::<Vec<char>>();

            let mut flag = true;

            for _ in 0..digits.len() {
                digits.rotate_right(1);
                let x =
                    u64::from_str_radix(&digits.clone().iter().collect::<String>(), 10).unwrap();

                if !x.is_prime() {
                    flag = false;
                    break;
                }
            }

            if flag {
                count += 1;
            }
        }
    }

    println!("[SOLUTION] circular primes below {}: {}", limit, count);
}
