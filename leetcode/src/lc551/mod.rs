#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool
    {
        let s_chars: Vec<char> = s.chars().collect();
        let mut abs_cnt = 0;
        let mut late_cnt = 0;
        let mut record = true;
        let len = s_chars.len();

        let mut i = 0;
        while i < len && record
        {
            let c = s_chars[i];
            match c {
                'L' => late_cnt += 1,
                'A' => {
                    late_cnt = 0;
                    abs_cnt += 1;
                },
                _ => late_cnt = 0,
            }

            if late_cnt == 3 || abs_cnt == 2 {
                record = false;
            }

            i += 1; 
        }
        
        record
    }
}
