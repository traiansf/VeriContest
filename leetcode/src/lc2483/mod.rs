#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
	pub fn best_closing_time(customers: String) -> i32 {
		let customers_chars: Vec<char> = customers.chars().collect();
		let len = customers_chars.len();
		let mut i: usize = 0;
		let mut score: i32 = 0;
		let mut best_score: i32 = 0;
		let mut best_hour: usize = 0;

		while i < len {
			let c = customers_chars[i];
			if c == 'Y' {
				score = score + 1;
			} else {
				score = score - 1;
			}
			if best_score < score {
				best_score = score;
				best_hour = i + 1;
			}
			i = i + 1;
		}

		best_hour as i32
	}
}
