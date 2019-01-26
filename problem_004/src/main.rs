use common::math::IsPalindrome;

fn main() {
    let mut largest: u32 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let current = x * y;

            if current.is_palindrome() && current > largest {
                largest = current;
            }
        }
    }

    println!("[SOLUTION] largest palindrome: {}", largest);
}
