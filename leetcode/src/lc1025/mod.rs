#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}
