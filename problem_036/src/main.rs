use common::format::ToBinary;
use common::input;
use common::math::IsPalindrome;

fn main() {
    println!("[INPUT] calculate until:");
    let limit = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    let mut sum = 0;

    for n in 0..limit {
        if n.is_palindrome() && n.to_binary().is_palindrome() {
            sum += n;
        }
    }

    println!(
        "[SOLUTION] sum of palindromic numbers in base 10 and 2 under {}: {}",
        limit, sum
    );
}
