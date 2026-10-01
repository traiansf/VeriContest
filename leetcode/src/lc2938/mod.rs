#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let s_chars: Vec<char> = s.chars().collect();
        let len = s_chars.len();
        let mut i: usize = 0;
        let mut ones: i64 = 0;
        let mut steps: i64 = 0;

        while i < len {
            let c = s_chars[i];
            if c == '1' {
                ones = ones + 1;
            } else {
                steps = steps + ones;
            }
            i = i + 1;
        }

        steps
    }
}
