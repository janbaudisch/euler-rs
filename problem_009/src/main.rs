use common::input;

fn main() {
    println!("[INPUT] a + b + c = ?");
    let limit = u64::from_str_radix(&input::read_line(), 10).expect("Could not parse input!");

    let mut product = 0;

    'outer: for a in 1..limit {
        for b in 1..limit {
            for c in 1..limit {
                if a + b + c == limit && a.pow(2) + b.pow(2) == c.pow(2) {
                    product = a * b * c;
                    break 'outer;
                }
            }
        }
    }

    println!("[SOLUTION] a * b * c = {}", product);
}
