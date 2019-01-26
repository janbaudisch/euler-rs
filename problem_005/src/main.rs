fn main() {
    let mut n = 1;

    loop {
        let mut flag = true;

        for i in 1..20 {
            if n % i != 0 {
                flag = false;
            }
        }

        if flag {
            break;
        }

        n += 1;
    }

    println!("[SOLUTION] smallest possible number: {}", n);
}
