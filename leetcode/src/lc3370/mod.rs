#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let target = n + 1;
        let mut p = 1;
        while p < target {
            p = p * 2;
        }
        p - 1
    }
}
