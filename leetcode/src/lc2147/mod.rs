#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor_chars: Vec<char> = corridor.chars().collect();
        let mod_num: u128 = 1_000_000_007;
        let len = corridor_chars.len();
        let mut seat_count: usize = 0;
        let mut plants: usize = 0;
        let mut ways: u128 = 1;
        let mut i: usize = 0;

        while i < len {
            let c = corridor_chars[i];
            if c == 'S' {
                if seat_count >= 2 && seat_count % 2 == 0 {
					let sep = plants + 1;
					let prod = ways * sep as u128;
					let new_ways = prod % mod_num;
					ways = new_ways;
                }
                seat_count += 1;
                plants = 0;
            } else {
                if seat_count >= 2 && seat_count % 2 == 0 {
                    plants += 1;
                }
            }
            i += 1;
        }

        if seat_count == 0 || seat_count % 2 == 1 {
            0
        } else {
            ways as i32
        }
    }
}
