#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let len = s_chars.len();
        let mut i: usize = 0;
        let mut ans: i32 = 0;

        while i < len {
            let a = s_chars[i];
            let b = s_chars[i + 1];
            if a != b {
                ans = ans + 1;
            }
            i = i + 2;
        }

        ans
    }
}
