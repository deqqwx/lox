#[cfg(test)]
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });          // Діапазон від 1 до 5 (5 не включений)
    assert_eq!((1..=5), RangeInclusive::new(1, 5));         // Діапазон від 1 до 5 (5 включений)

    println!("Success!");
}
