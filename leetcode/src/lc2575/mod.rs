#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let word_chars: Vec<char> = word.chars().collect();
        let n = word_chars.len();
        let mm: i64 = m as i64;
        let mut rem: i64 = 0;
        let mut i: usize = 0;
        let mut res: Vec<i32> = Vec::new();

        while i < n {
            let d = (word_chars[i] as i64) - ('0' as i64);
            rem = (rem * 10 + d) % mm;
            if rem == 0 {
                res.push(1);
            } else {
                res.push(0);
            }
            i = i + 1;
        }

        res
    }
}
