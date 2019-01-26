pub trait IsPrime {
    fn is_prime(self) -> bool;
}

pub fn nth_prime(count: usize) -> u64 {
    let mut current;
    let mut i = 0;
    let mut n = 2;

    loop {
        if n.is_prime() {
            current = n;
            i += 1;

            if i == count {
                break;
            }
        }

        n += 1;
    }

    current
}

macro_rules! impl_is_prime {
    ($type: ident) => {
        impl IsPrime for $type {
            fn is_prime(self) -> bool {
                for n in 2..=self / 2 {
                    if self % n == 0 {
                        return false;
                    }
                }

                true
            }
        }
    };
}

impl_is_prime!(u8);
impl_is_prime!(u16);
impl_is_prime!(u32);
impl_is_prime!(u64);
impl_is_prime!(u128);
