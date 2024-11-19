use crate::files::{HashedFile, HashedFileList};
use std::fs::File;
use std::io::{self, Write};

pub fn ctn_file_cksum_gnu(ctn_file: &mut File, hashed_list: &HashedFileList) -> io::Result<()> {
	for file in hashed_list.get_files() {
		let line = format_line(&file);
		ctn_file.write_all(line.as_bytes())?;
	}
	Ok(())
}

#[inline]
fn format_line(file: &HashedFile) -> String {
	format!(
		"{} *{}\n",
		file.get_hash(),
		file.get_relative_path().display(),
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hash::HashFunc;
	use std::path::PathBuf;

	#[test]
	fn simple_sha256() {
		let file = HashedFile::new(
			PathBuf::from("test_file.txt"),
			42,
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			HashFunc::Sha256,
		);
		let line = format_line(&file);
		let ref_line =
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 *test_file.txt\n";
		assert_eq!(line, ref_line.to_string());
	}

	#[test]
	fn simple_sha3_512() {
		let file = HashedFile::new(
			PathBuf::from("test_file.txt"),
			42,
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			HashFunc::Sha3_512,
		);
		let line = format_line(&file);
		let ref_line =
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 *test_file.txt\n";
		assert_eq!(line, ref_line.to_string());
	}

	#[test]
	fn space_sha384() {
		let file = HashedFile::new(
			PathBuf::from("test file.txt"),
			42,
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			HashFunc::Sha384,
		);
		let line = format_line(&file);
		let ref_line =
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 *test file.txt\n";
		assert_eq!(line, ref_line.to_string());
	}

	#[test]
	fn test_par_sha256() {
		let file = HashedFile::new(
			PathBuf::from("(test_file)(01).txt"),
			42,
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			HashFunc::Sha256,
		);
		let line = format_line(&file);
		let ref_line = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 *(test_file)(01).txt\n";
		assert_eq!(line, ref_line.to_string());
	}

	#[test]
	fn test_space_par_blake3() {
		let file = HashedFile::new(
			PathBuf::from("  (test file)(01).txt"),
			42,
			"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
			HashFunc::Blake3,
		);
		let line = format_line(&file);
		let ref_line = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 *  (test file)(01).txt\n";
		assert_eq!(line, ref_line.to_string());
	}
}
