pub trait Factorial {
    fn factorial(self) -> u64;
}

macro_rules! impl_factorial {
    ($type: ident) => {
        impl Factorial for $type {
            fn factorial(self) -> u64 {
                let mut result: u64 = 1;

                match self {
                    0 | 1 => return 1,
                    2 => return 2,
                    _ => {
                        for n in 2..(self + 1) as u64 {
                            result *= n;
                        }
                    }
                }

                return result;
            }
        }
    };
}

impl_factorial!(u8);
impl_factorial!(u16);
impl_factorial!(u32);
impl_factorial!(u64);
impl_factorial!(u128);
