use crate::analyse_hash::analyse_hash;
use crate::file::File;
use crate::hasher::HashFunc;
use nom::character::complete::{alphanumeric1, char, not_line_ending, one_of};
use nom::combinator::fail;
use nom::IResult;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const DEFAULT_HASH: HashFunc = HashFunc::Sha256;
const DEFAULT_CAPACITY: usize = 256;

pub fn cksum_gnu_get_files(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<(Vec<File>, HashFunc), ()> {
	let mut files = Vec::with_capacity(DEFAULT_CAPACITY);
	let file = std::fs::File::open(path).map_err(|_| ())?;
	for line in BufReader::new(file).lines() {
		files.push(get_file(&line.map_err(|_| ())?)?);
	}
	let hash = match files.first() {
		Some(f) => match f.get_hash() {
			Some(h) => analyse_hash(h, DEFAULT_HASH),
			None => DEFAULT_HASH,
		},
		None => {
			return Err(());
		}
	};
	Ok((files, hash))
}

fn get_file(content: &str) -> Result<File, ()> {
	let (_, file) = parse_line(content).map_err(|_| ())?;
	Ok(file)
}

fn parse_line(input: &str) -> IResult<&str, File> {
	let (input, hash) = alphanumeric1(input)?;
	let (input, _) = char(' ')(input)?;
	let (input, _) = one_of(" *")(input)?;
	let (input, path) = not_line_ending(input)?;
	if path.is_empty() {
		let _: (&str, File) = fail(input)?;
	}
	let path = PathBuf::from(path);
	Ok((input, File::create_dummy(&path, &PathBuf::new(), 0, hash)))
}

#[cfg(test)]
mod tests {
	use super::parse_line;
	use std::path::Path;

	#[test]
	fn simple_t() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_t_lf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_t_crlf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  test_file.txt\r\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b_lf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_b_crlf() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d *test_file.txt\r\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test_file.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn filename_with_space() {
		let res = parse_line(
			"11586d2eb43b73e539caa3d158c883336c0e2c904b309c0c5ffe2c9b83d562a1  test file 01.txt",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("test file 01.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"11586d2eb43b73e539caa3d158c883336c0e2c904b309c0c5ffe2c9b83d562a1"
		);
	}

	#[test]
	fn filename_with_accent() {
		let res =
			parse_line("f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2  è_é.txt");
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_path(), Path::new("è_é.txt"));
		let hash = file.get_hash();
		assert!(hash.is_some());
		let hash = hash.unwrap();
		assert_eq!(
			hash,
			"f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2"
		);
	}

	#[test]
	fn invalid_hash() {
		let res = parse_line(
			"1c527c66%fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d test_file.txt\n",
		);
		assert!(res.is_err());
	}

	#[test]
	fn invalid_mode() {
		let res = parse_line(
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d test_file.txt\n",
		);
		assert!(res.is_err());
	}

	#[test]
	fn no_file_name() {
		let res =
			parse_line("1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d  \n");
		assert!(res.is_err());
	}

	#[test]
	fn empty() {
		let res = parse_line("");
		assert!(res.is_err());
	}
}
