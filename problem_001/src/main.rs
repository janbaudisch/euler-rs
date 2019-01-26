use common::input;

fn main() {
    println!("[INPUT] calculate until:");
    let input = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");
    let mut multiples: Vec<u64> = Vec::new();

    for n in 0..input {
        if n % 3 == 0 || n % 5 == 0 {
            multiples.push(n);
        }
    }

    println!(
        "[SOLUTION] sum of multiples: {}",
        multiples.iter().sum::<u64>()
    );
}
