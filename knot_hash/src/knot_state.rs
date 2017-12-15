use ::DenseHash;

#[derive(Copy, Clone)]
pub struct KnotState {
	pub list: [u8; 256],
	pub position: usize,
	pub skip: usize
}

impl Default for KnotState {
	fn default() -> KnotState {
		let mut list = [0; 256];
		for (dst, src) in list.iter_mut().zip(0usize..) {
			*dst = src as u8;
		}
        
		KnotState {
			list: list, position: 0, skip: 0 }
		}
}

impl KnotState {
	pub fn new() -> KnotState { Default::default() }

	pub fn dense_hash(&self) -> DenseHash {
		use std::ops::BitXor;
		let mut result = [0; 16];
		for i in 0..16 {
			let range = (i*16)..((i+1)*16);
			result[i] = (self.list[range]).iter().fold(0, BitXor::bitxor);
		}

		DenseHash::new(result)
	}

	pub fn perform_swap(mut self, length: usize) -> KnotState {
		let swaps = length / 2;
		let start = self.position;
		let end = start + length - 1;
		for offset in 0..swaps {
			let a = (start + offset) % self.list.len();
			let b = (end - offset) % self.list.len();
			self.list.swap(a, b);
		}

		self.position = (self.position + length + self.skip) % self.list.len();
		self.skip = (self.skip + 1) % self.list.len();
		self
	}

	pub fn perform_round<T: IntoIterator<Item=usize>>(self, lengths: T) -> KnotState {
		lengths.into_iter().fold(self, KnotState::perform_swap)
	}

	pub fn perform_rounds<T: IntoIterator<Item=usize> + Clone>(self, lengths: T, rounds: usize) -> KnotState {
		::std::iter::repeat(lengths).take(rounds).fold(self, KnotState::perform_round)
	}
}