mod date;

use date::*;

fn main() {
    let mut current = Date {
        weekday: Weekday::Monday,
        day: 1,
        month: Month::January,
        year: 1900,
    };

    let mut count = 0;

    loop {
        current = current.next();

        if current.year > 1900 {
            break;
        }
    }

    loop {
        if current.day == 1 && current.weekday == Weekday::Sunday {
            count += 1;
        }

        current = current.next();

        if current.year > 2000 {
            break;
        }
    }

    println!("[SOLUTION] {} Sundays on the first of the month", count);
}
