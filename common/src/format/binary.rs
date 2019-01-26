#[derive(Clone, Copy, PartialEq)]
pub enum Bit {
    One,
    Zero
}

pub trait ToBinary {
    fn to_binary(self) -> Vec<Bit>;
}

macro_rules! impl_to_binary {
    ($type: ident) => {
        impl ToBinary for $type {
            fn to_binary(self) -> Vec<Bit> {
                let mut n: $type = self;
                let mut bits = Vec::new();

                loop {
                    if n == 0 {
                        break;
                    }

                    let remainder = n % 2;

                    match remainder {
                        0 => bits.push(Bit::Zero),
                        1 => bits.push(Bit::One),
                        _ => bits.push(Bit::Zero)
                    }

                    n /= 2;
                }

                bits.reverse();

                bits
            }
        }
    };
}

impl_to_binary!(u8);
impl_to_binary!(u16);
impl_to_binary!(u32);
impl_to_binary!(u64);
impl_to_binary!(u128);
