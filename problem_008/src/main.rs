use common::input;

fn main() {
    println!("[INPUT] number (multi-line):");
    let input: Vec<String> = input::read_lines();
    let digits = input
        .iter()
        .fold(String::new(), |string, part| string + part)
        .chars()
        .map(|digit| digit.to_digit(10).expect("Could not parse input!"))
        .collect::<Vec<u32>>();

    let mut largest = 0;

    for start in 0..digits.len() - 12 {
        let mut product = 1;

        for i in 0..13 {
            product *= u64::from(digits[start + i]);
        }

        if product > largest {
            largest = product;
        }
    }

    println!(
        "[SOLUTION] largest product of 13 adjacent digits: {}",
        largest
    );
}
