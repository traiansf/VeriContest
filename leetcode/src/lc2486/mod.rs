#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let s_len = s_chars.len();
        let t_len = t_chars.len();
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < s_len {
            let c = s_chars[i];
            if j < t_len && c == t_chars[j] {
                j = j + 1;
            }
            i = i + 1;
        }

        (t_len - j) as i32
    }
}
