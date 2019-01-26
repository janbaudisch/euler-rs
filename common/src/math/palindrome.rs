pub trait IsPalindrome {
    fn is_palindrome(self) -> bool;
}

macro_rules! impl_is_palindrome {
    ($type: ident) => {
        impl IsPalindrome for $type {
            fn is_palindrome(self) -> bool {
                let digits = self.to_string().chars().collect::<Vec<char>>();

                let (left, right) = if &digits.len() % 2 == 0 {
                    digits.split_at(digits.len() / 2)
                } else {
                    digits.split_at((digits.len() - 1) / 2)
                };

                let mut right = right.to_vec();
                right.reverse();

                left == right.as_slice()
            }
        }
    };
}

impl_is_palindrome!(u8);
impl_is_palindrome!(u16);
impl_is_palindrome!(u32);
impl_is_palindrome!(u64);
impl_is_palindrome!(u128);
