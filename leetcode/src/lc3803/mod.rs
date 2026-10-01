#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    fn is_first_at(s: &[char], p: usize) -> bool {
        let mut q: usize = 0;
        while q < p {
            if s[q] == s[p] {
                return false;
            }
            q += 1;
        }
        true
    }

    pub fn residue_prefixes(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut ans: i32 = 0;
        let mut i: usize = 0;
        while i < n {
            let mut distinct: i32 = 0;
            let mut p: usize = 0;
            while p <= i {
                let is_new = Self::is_first_at(&s, p);
                if is_new {
                    distinct += 1;
                }
                p += 1;
            }

            let residue: i32 = ((i + 1) % 3) as i32;
            if distinct == residue {
                ans += 1;
            }
            i += 1;
        }
        ans
    }
}
