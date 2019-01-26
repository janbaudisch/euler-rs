fn main() {
    let mut sum: u32 = 0;
    let mut square: u32 = 0;

    for n in 0..101_u32 {
        sum += n.pow(2);
        square += n;
    }

    println!("[SOLUTION] difference: {}", square.pow(2) - sum);
}
