use crate::hash::{HashFunc, CHARS_TO_REMOVE};
use std::path::Path;
use strum::IntoEnumIterator;

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

pub fn from_path(path: &Path) -> Option<HashFunc> {
	if let Some(raw_name) = path.file_name() {
		if let Some(name) = raw_name.to_str() {
			for hash_func in HashFunc::iter() {
				let hash_name = hash_func
					.to_string()
					.replace(CHARS_TO_REMOVE, "")
					.to_ascii_lowercase();
				let file_name = name.replace(CHARS_TO_REMOVE, "").to_ascii_lowercase();
				if file_name.contains(&hash_name) {
					return Some(hash_func);
				}
			}
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::{analyse_hash, from_path, HashFunc};
	use std::path::PathBuf;

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

	#[test]
	fn test_from_path() {
		let tests = &[
			("", None),
			("/home/test/", None),
			("/home/test/contenu.txt", None),
			("/home/test/skein256sum.txt", None),
			("sha256sums.txt", Some(HashFunc::Sha256)),
			("/home/test/sha256sums.txt", Some(HashFunc::Sha256)),
			("/home/test/Sha-3_384sums.txt", Some(HashFunc::Sha3_384)),
			("/home/test/SHA-512_sums.txt", Some(HashFunc::Sha512)),
			("/home/test/sums_blake2_B.txt", Some(HashFunc::Blake2b)),
			("/home/test/Blake 3 sums.txt", Some(HashFunc::Blake3)),
		];
		for (path_str, expected_hash) in tests {
			let path = PathBuf::from(path_str);
			let hash = from_path(path.as_path());
			assert_eq!(hash, *expected_hash);
		}
	}
}
