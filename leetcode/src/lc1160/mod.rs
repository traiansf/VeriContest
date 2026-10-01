#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars_str: Vec<char> = chars.chars().collect();
        let chars_len = chars_str.len();
        let mut chars_count: Vec<i32> = Vec::new();
        let mut i = 0;
        
        while i < 26 {
            chars_count.push(0);
            i += 1;
        }

        i = 0;
        while i < chars_len {
            let c = chars_str[i];
            let idx = (c as u32 - 97) as usize;
            chars_count[idx] = chars_count[idx] + 1;
            i += 1;
        }

        let mut sum: i32 = 0;
        let mut k = 0;
        while k < words.len() {
            let word_str: Vec<char> = words[k].chars().collect();
            let word_len = word_str.len();
            
            let mut word_count: Vec<i32> = Vec::new();
            let mut j = 0;
            while j < 26 {
                word_count.push(0);
                j += 1;
            }

            j = 0;
            while j < word_len {
                let c = word_str[j];
                let idx = (c as u32 - 97) as usize;
                word_count[idx] = word_count[idx] + 1;
                j += 1;
            }

            let mut can = true;
            j = 0;
            while j < 26 {
                if word_count[j] > chars_count[j] {
                    can = false;
                }
                j += 1;
            }

            if can {
                sum += word_len as i32;
            }
            
            k += 1;
        }

        sum
    }
}
