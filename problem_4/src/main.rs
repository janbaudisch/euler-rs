fn check_palindrome(input: u32) -> bool {
    let digits = input.to_string().chars().collect::<Vec<char>>();

    let (left, right) = if &digits.len() % 2 == 0 {
        digits.split_at(digits.len() / 2)
    } else {
        digits.split_at((digits.len() - 1) / 2)
    };

    let mut right = right.to_vec();
    right.reverse();

    left == right.as_slice()
}

fn main() {
    let mut largest: u32 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let current = x * y;
            if check_palindrome(current) && current > largest {
                largest = current;
            }
        }
    }

    println!("[SOLUTION] largest palindrome: {}", largest);
}
