#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
	pub fn winner_of_game(colors: String) -> bool {
		let colors_chars: Vec<char> = colors.chars().collect();
		let len = colors_chars.len();
		if len < 3 {
			return false;
		}

		let mut i: usize = 1;
		let mut alice: i32 = 0;
		let mut bob: i32 = 0;

		while i + 1 < len {
			let left = colors_chars[i - 1];
			let mid = colors_chars[i];
			let right = colors_chars[i + 1];
			if left == 'A' && mid == 'A' && right == 'A' {
				alice = alice + 1;
			}
			if left == 'B' && mid == 'B' && right == 'B' {
				bob = bob + 1;
			}
			i = i + 1;
		}

		alice > bob
	}
}
