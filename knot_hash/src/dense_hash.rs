#[derive(Copy, Clone, PartialEq, Eq)]
pub struct DenseHash {
    storage: [u8; 32]
}

impl DenseHash {
    pub fn new(bytes: [u8; 16]) -> DenseHash {
        let mut storage = [0; 32];
        for i in 0..32 {
            let byte = bytes[i/2];
            let nibble = if (i & 1) == 0 {
                byte >> 4
            } else {
                byte & 0x0f
            };
            storage[i] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'][nibble as usize] as u8;
        }

        DenseHash { storage }
    }
}

impl ::std::ops::Deref for DenseHash {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { ::std::str::from_utf8_unchecked(&self.storage) }
    }
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_dense_hash() {
		let expected = "0123456789abcdef0000000000000000";
		let input = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0, 0, 0, 0, 0, 0, 0, 0];
		let actual = ::DenseHash::new(input);
		assert_eq!(expected, &*actual);
	}
}