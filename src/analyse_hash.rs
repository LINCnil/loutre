use crate::hash::HashFunc;

macro_rules! get_hash {
	($choices: ident, $default: ident, $fallback: expr) => {
		if $choices.contains(&$default) {
			$default
		} else {
			$fallback
		}
	};
}

const LEN_64: &[HashFunc] = &[
	HashFunc::Sha256,
	HashFunc::Sha3_256,
	HashFunc::Blake2s,
	HashFunc::Blake3,
];
const LEN_96: &[HashFunc] = &[HashFunc::Sha384, HashFunc::Sha3_384];
const LEN_128: &[HashFunc] = &[HashFunc::Sha512, HashFunc::Sha3_512, HashFunc::Blake2b];

pub fn analyse_hash(hash_str: &str, default: HashFunc) -> HashFunc {
	match hash_str.len() {
		64 => get_hash!(LEN_64, default, HashFunc::Sha256),
		96 => get_hash!(LEN_96, default, HashFunc::Sha384),
		128 => get_hash!(LEN_128, default, HashFunc::Sha512),
		_ => default,
	}
}

#[cfg(test)]
mod tests {
	use super::{analyse_hash, HashFunc};

	const HASH_256: &str = "c28a75847bf2a6b53985d7527973bf8bf2e76a5ff0781e63edc9afeab16e1a2a";
	const HASH_512: &str = "55a3017c0eb4be180c2c7bc055ab5e5cc3e8a4ea9d93c517dea4a35fc86efc5fc1723386d2e55752b650b0e2636718cd281fe5568f666d5a9b671d875853d2b2";

	#[test]
	fn default_unknown() {
		let hash = analyse_hash("", HashFunc::Sha256);
		assert_eq!(hash, HashFunc::Sha256);
	}

	#[test]
	fn sha256() {
		let hash = analyse_hash(HASH_256, HashFunc::Sha512);
		assert_eq!(hash, HashFunc::Sha256);
	}

	#[test]
	fn sha256_default() {
		let hash = analyse_hash(HASH_256, HashFunc::Sha256);
		assert_eq!(hash, HashFunc::Sha256);
	}

	#[test]
	fn blake_2b() {
		let hash = analyse_hash(HASH_512, HashFunc::Blake2b);
		assert_eq!(hash, HashFunc::Blake2b);
	}
}
