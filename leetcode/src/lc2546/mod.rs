#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        let s_len = s_chars.len();
        let t_len = target_chars.len();
        let mut i: usize = 0;
        let mut has_s: bool = false;
        while i < s_len {
            if s_chars[i] == '1' {
                has_s = true;
            }
            i = i + 1;
        }

        i = 0;
        let mut has_t: bool = false;
        while i < t_len {
            if target_chars[i] == '1' {
                has_t = true;
            }
            i = i + 1;
        }

        has_s == has_t
    }
}
