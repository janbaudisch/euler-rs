pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn fibonacci_sequence(until: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = Vec::new();

    let mut n = 0;

    loop {
        sequence.push(match n {
            0 => 1,
            1 => 1,
            _ => {
                let fib = sequence[(n - 1) as usize] + sequence[(n - 2) as usize];

                if fib > until {
                    break;
                }

                fib
            }
        });

        n += 1;
    }

    sequence
}
