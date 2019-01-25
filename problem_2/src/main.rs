use common::{input, math};

fn main() {
    let until = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    println!(
        "[SOLUTION] sum: {}",
        math::fibonacci_sequence(until)
            .iter()
            .filter(|x| *x % 2 == 0)
            .sum::<u64>()
    );
}
