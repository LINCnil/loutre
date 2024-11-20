use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use nom::character::complete::{alphanumeric1, char, hex_digit1, line_ending, none_of};
use nom::combinator::{eof, fail, opt};
use nom::multi::{fold_many0, many0};
use nom::IResult;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn cksum_bsd_get_files(path: &Path, _default_hash: HashFunc) -> Result<HashedFileList, ()> {
	let mut files = HashedFileList::new();
	let rcpt_file = std::fs::File::open(path).map_err(|_| ())?;
	for line in BufReader::new(rcpt_file).lines() {
		let file = get_file(&line.map_err(|_| ())?)?;
		files.insert_file(file);
	}
	Ok(files)
}

fn get_file(content: &str) -> Result<HashedFile, ()> {
	let (_, file) = parse_line(content).map_err(|_| ())?;
	Ok(file)
}

fn parse_line(input: &str) -> IResult<&str, HashedFile> {
	let (input, hash_func) = parse_hash_func(input)?;
	let (input, _) = char(' ')(input)?;
	let (input, path) = parse_file_name(input)?;
	let (input, _) = char(' ')(input)?;
	let (input, _) = char('=')(input)?;
	let (input, _) = char(' ')(input)?;
	let (input, hash) = parse_fingerprint(input)?;
	let (input, _) = opt(line_ending)(input)?;
	let (input, _) = eof(input)?;
	let file = HashedFile::new(path, 0, hash, hash_func);
	Ok((input, file))
}

fn parse_hash_func(input: &str) -> IResult<&str, HashFunc> {
	let (input, hash_func_name) = alphanumeric1(input)?;
	let res = HashFunc::from_str(hash_func_name);
	if res.is_err() {
		let _: (&str, HashFunc) = fail(input)?;
	}
	let hash_func = res.unwrap();
	Ok((input, hash_func))
}

fn parse_file_name(input: &str) -> IResult<&str, PathBuf> {
	let (input, _) = char('(')(input)?;
	let (input, path_vec) = many0(parse_path_part)(input)?;
	let path = path_vec.join(")");
	if path.is_empty() {
		let _: (&str, PathBuf) = fail(input)?;
	}
	Ok((input, Path::new(&path).to_path_buf()))
}

fn parse_path_part(input: &str) -> IResult<&str, String> {
	let (input, path_part) = fold_many0(none_of(")"), String::new, |mut acc: String, item| {
		acc.push(item);
		acc
	})(input)?;
	let (input, _) = char(')')(input)?;
	Ok((input, path_part))
}

fn parse_fingerprint(input: &str) -> IResult<&str, &str> {
	hex_digit1(input)
}

#[cfg(test)]
mod tests {
	use super::parse_line;
	use crate::hash::HashFunc;
	use std::path::Path;

	#[test]
	fn alnum() {
		let res = parse_line(
			"SHA256 (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn filename_with_accent() {
		let res = parse_line(
			"SHA256 (è_é.txt) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("è_é.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn alnum_lf() {
		let res = parse_line(
			"SHA256 (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn alnum_crlf() {
		let res = parse_line(
			"SHA256 (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d\r\n",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_sha256() {
		let res = parse_line(
			"SHA256 (test_file.txt) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn simple_blake2b() {
		let res = parse_line(
			"BLAKE2b (test_file.txt) = a71079d42853dea26e453004338670a53814b78137ffbed07603a41d76a483aa9bc33b582f77d30a65e6f29a896c0411f38312e1d66e0bf16386c86a89bea572",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Blake2b);
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(
			file.get_hash(),
			"a71079d42853dea26e453004338670a53814b78137ffbed07603a41d76a483aa9bc33b582f77d30a65e6f29a896c0411f38312e1d66e0bf16386c86a89bea572"
		);
	}

	#[test]
	fn par_1() {
		let res = parse_line(
			"SHA256 (test file (01).txt) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test file (01).txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn par_2() {
		let res = parse_line(
			"SHA256 (test_file_(01).txt)))) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new("test_file_(01).txt)))"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn par_3() {
		let res = parse_line(
			"SHA256 ()test_file_(01).txt) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_hash_func(), HashFunc::Sha256);
		assert_eq!(file.get_relative_path(), Path::new(")test_file_(01).txt"));
		assert_eq!(
			file.get_hash(),
			"1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d"
		);
	}

	#[test]
	fn invalid_hash_func() {
		let res = parse_line(
			"INVALID (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_err());
	}

	#[test]
	fn invalid_no_file_name() {
		let res = parse_line(
			"SHA256 () = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_err());
	}

	#[test]
	fn invalid_hash() {
		let res = parse_line(
			"SHA256 (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafgggggg",
		);
		assert!(res.is_err());
	}

	#[test]
	fn invalid_additional_data() {
		let res = parse_line(
			"SHA256 (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d invalid",
		);
		assert!(res.is_err());
	}

	#[test]
	fn no_hash_func() {
		let res = parse_line(
			" (test) = 1c527c669fdb2cdb8a7d3e75137bbb71b5e53ddad14e9fe6c0691f8fafb6893d",
		);
		assert!(res.is_err());
	}

	#[test]
	fn no_hash() {
		let res = parse_line("SHA256 (test) = ");
		assert!(res.is_err());
	}

	#[test]
	fn empty() {
		let res = parse_line("");
		assert!(res.is_err());
	}
}
