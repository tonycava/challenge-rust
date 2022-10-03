use std::iter::Sum;

pub fn sum<T>(a: T) -> i32 where T: Iterator, i32: Sum<<T as Iterator>::Item> {
    return a.into_iter().sum();
}

pub fn thirtytwo_tens() -> [i32; 32] {
    return [10; 32];
}