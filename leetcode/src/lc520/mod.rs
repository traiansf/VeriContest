#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool
    {
        let word_chars: Vec<char> = word.chars().collect();
        let len = word_chars.len();
        let mut all_upper = true;
        let mut i: usize = 0;

        while i < len && all_upper
        {
            let c = word_chars[i];
            if !(c >= 'A' && c <= 'Z') {
                all_upper = false;
            }
            i += 1;
        }
        
        if all_upper {
            return true;
        }
        
        let mut all_lower = true;
        i = 0;
        while i < len && all_lower
        {
            let c = word_chars[i];
            if !(c >= 'a' && c <= 'z') {
                all_lower = false;
            }
            i += 1;
        }
        
        if all_lower {
            return true;
        }
        
        let first = word_chars[0];
        if !(first >= 'A' && first <= 'Z') {
            return false;
        }
                
        i = 1;
        let mut rest_lower = true;
        while i < len && rest_lower
        {
            let c = word_chars[i];
            if !(c >= 'a' && c <= 'z') {
                rest_lower = false;
            }
            i += 1;
        }
        
        rest_lower
    }
}
