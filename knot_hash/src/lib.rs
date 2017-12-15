pub use dense_hash::DenseHash;
pub use knot_state::KnotState;

mod dense_hash;
mod knot_state;

pub fn knot_hash(input: &[u8]) -> DenseHash {
	let mut real_input = Vec::with_capacity(input.len() + 5);
	real_input.extend(input);
	real_input.extend(&[17, 31, 73, 47, 23][..]);
	knot_hash_no_suffix(&real_input)
}

pub fn knot_hash_no_suffix(input: &[u8]) -> DenseHash {
	fn fix_length(&length: &u8) -> usize { length as usize }
	KnotState::new().perform_rounds(input.iter().map(fix_length), 64).dense_hash()
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_empty() {
		let expected = "a2582a3a0e66e6e86e3812dcb672a272";
		let actual = ::knot_hash(&[]);
		assert_eq!(expected, &*actual);
	}

	#[test]
	fn test_123() {
		let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
		let actual = ::knot_hash("1,2,3".as_bytes());
		assert_eq!(expected, &*actual);
	}

	#[test]
	fn test_124() {
		let expected = "63960835bcdc130f0b66d7ff4f6a5a8e";
		let actual = ::knot_hash("1,2,4".as_bytes());
		assert_eq!(expected, &*actual);
	}

	#[test]
	fn test_aoc2017() {
		let expected = "33efeb34ea91902bb2f59c9920caa6cd";
		let actual = ::knot_hash("AoC 2017".as_bytes());
		assert_eq!(expected, &*actual);
	}

	#[test]
	fn test_new() {
		let ks = ::KnotState::new();
		assert_eq!(0, ks.position);
		assert_eq!(0, ks.skip);
		for i in 0..256 {
			assert_eq!(i as u8, ks.list[i]);
		}
	}
}