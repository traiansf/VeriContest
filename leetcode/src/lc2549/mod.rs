#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            n - 1
        }
    }
}
