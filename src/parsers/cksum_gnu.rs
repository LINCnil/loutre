use crate::analyse_hash::analyse_hash;
use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use nom::character::complete::{alphanumeric1, char, not_line_ending, one_of};
use nom::combinator::fail;
use nom::IResult;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn cksum_gnu_get_files(path: &Path, default_hash: HashFunc) -> Result<HashedFileList, ()> {
	let mut files = HashedFileList::new();
	let mut hash_func = None;
	let rcpt_file = std::fs::File::open(path).map_err(|_| ())?;
	for line in BufReader::new(rcpt_file).lines() {
		let file = get_file(&line.map_err(|_| ())?, hash_func, default_hash)?;
		if hash_func.is_none() {
			hash_func = Some(file.get_hash_func());
		}
		files.insert_file(file);
	}
	Ok(files)
}

fn get_file(
	content: &str,
	hash_func: Option<HashFunc>,
	default_hash: HashFunc,
) -> Result<HashedFile, ()> {
	let (_, file) = parse_line(content, hash_func, default_hash).map_err(|_| ())?;
	Ok(file)
}

fn parse_line(
	input: &str,
	hash_func: Option<HashFunc>,
	default_hash: HashFunc,
) -> IResult<&str, HashedFile> {
	let (input, hash) = alphanumeric1(input)?;
	let (input, _) = char(' ')(input)?;
	let (input, _) = one_of(" *")(input)?;
	let (input, path) = not_line_ending(input)?;
	if path.is_empty() {
		let _: (&str, HashedFile) = fail(input)?;
	}
	let path = PathBuf::from(path);
	let hash_func = match hash_func {
		Some(h) => h,
		None => analyse_hash(hash, default_hash),
	};
	let file = HashedFile::new(path, 0, hash, hash_func);
	Ok((input, file))
}

#[cfg(test)]
mod tests {
	use super::parse_line;
	use crate::hash::HashFunc;
	use std::path::Path;

	#[test]
	fn simple_t() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_t_lf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_t_crlf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt\r\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b_lf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b_crlf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt\r\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn filename_with_space() {
		let res = parse_line(
			"11586d2eb43b73e539caa3d158c883336c0e2c904b309c0c5ffe2c9b83d562a1  test file 01.txt",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test file 01.txt"));
		assert_eq!(
			file.get_hash(),
			"11586d2eb43b73e539caa3d158c883336c0e2c904b309c0c5ffe2c9b83d562a1"
		);
	}

	#[test]
	fn filename_with_accent() {
		let res = parse_line(
			"f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2  è_é.txt",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("è_é.txt"));
		assert_eq!(
			file.get_hash(),
			"f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2"
		);
	}

	#[test]
	fn invalid_hash() {
		let res = parse_line(
			"1c527c66%fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d test_file.txt\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn invalid_mode() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d test_file.txt\n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn no_file_name() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  \n",
			None,
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn empty() {
		let res = parse_line("", None, HashFunc::Sha256);
		assert!(res.is_err());
	}
}
