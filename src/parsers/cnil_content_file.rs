use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use nom::character::complete::{hex_digit1, line_ending, none_of, tab, u64 as parse_u64};
use nom::combinator::{eof, fail, opt};
use nom::multi::many1;
use nom::{IResult, Parser};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn cnil_content_file_get_files(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<HashedFileList, ()> {
	let mut files = HashedFileList::new();
	let rcpt_file = std::fs::File::open(path).map_err(|_| ())?;
	let mut all_lines = BufReader::new(rcpt_file).lines();
	let first_line = all_lines.next().ok_or(())?.map_err(|_| ())?;
	let hash_func = get_header(&first_line)?;
	for line in all_lines {
		let file = get_file(&line.map_err(|_| ())?, hash_func)?;
		files.insert_file(file);
	}
	Ok(files)
}

fn get_header(content: &str) -> Result<HashFunc, ()> {
	let (_, hash_func) = parse_header(content).map_err(|_| ())?;
	Ok(hash_func)
}

fn parse_header(input: &str) -> IResult<&str, HashFunc> {
	let (input, _file_name) = parse_junk(input)?;
	let (input, _) = tab(input)?;
	let (input, _file_name) = parse_junk(input)?;
	let (input, _) = tab(input)?;
	let (input, hash_func) = parse_hash_func(input)?;
	let (input, _) = opt(tab).parse(input)?;
	let (input, _) = opt(line_ending).parse(input)?;
	let (input, _) = eof(input)?;
	Ok((input, hash_func))
}

fn parse_junk(input: &str) -> IResult<&str, String> {
	let (input, junk) = many1(none_of("\t")).parse(input)?;
	Ok((input, junk.iter().collect()))
}

fn parse_hash_func(input: &str) -> IResult<&str, HashFunc> {
	let (input, hash_func_name) = parse_junk(input)?;
	let res = HashFunc::from_str(&hash_func_name);
	if res.is_err() {
		let _: (&str, HashFunc) = fail().parse(input)?;
	}
	let hash_func = res.unwrap();
	Ok((input, hash_func))
}

fn get_file(content: &str, hash_func: HashFunc) -> Result<HashedFile, ()> {
	let (_, file) = parse_line(content, hash_func).map_err(|_| ())?;
	Ok(file)
}

fn parse_line(input: &str, hash_func: HashFunc) -> IResult<&str, HashedFile> {
	let (input, path) = parse_file_name(input)?;
	let (input, _) = tab(input)?;
	let (input, size) = parse_u64(input)?;
	let (input, _) = tab(input)?;
	let (input, hash) = parse_fingerprint(input)?;
	let (input, _) = opt(tab).parse(input)?;
	let (input, _) = opt(line_ending).parse(input)?;
	let (input, _) = eof(input)?;
	let file = HashedFile::new(path, size, hash, hash_func);
	Ok((input, file))
}

// File names can contains tabs, and since it used several times as the delimiting character, we
// have to use a few trics.
fn parse_file_name(input: &str) -> IResult<&str, PathBuf> {
	// Split on tabs.
	let mut parts: Vec<&str> = input.split('\t').collect();

	// Remove the elements that are not from the file name.
	let res = parts.pop();
	if res.is_none() {
		let _: (&str, PathBuf) = fail().parse(input)?;
	}
	if res.unwrap().is_empty() {
		// We have an extra tab at the end.
		if parts.pop().is_none() {
			let _: (&str, PathBuf) = fail().parse(input)?;
		}
	}
	if parts.pop().is_none() {
		let _: (&str, PathBuf) = fail().parse(input)?;
	}

	// Get the file path and check it.
	let path = parts.join("\t");
	if path.is_empty() {
		let _: (&str, HashedFile) = fail().parse(input)?;
	}

	// Remove the file path from the input.
	let input = match input.strip_prefix(&path) {
		Some(i) => i,
		None => input,
	};

	// We are done.
	let path = Path::new(&path).to_path_buf();
	Ok((input, path))
}

fn parse_fingerprint(input: &str) -> IResult<&str, &str> {
	hex_digit1(input)
}

#[cfg(test)]
mod tests {
	use super::{parse_header, parse_line};
	use crate::hash::HashFunc;
	use std::path::Path;

	#[test]
	fn header_sha256() {
		let res = parse_header("Nom du document\tTaille (octets)\tSHA256");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Sha256);
	}

	#[test]
	fn header_sha256_end_tab() {
		let res = parse_header("Nom du document\tTaille (octets)\tSHA256\t");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Sha256);
	}

	#[test]
	fn header_sha3_256() {
		let res = parse_header("Nom du document\tTaille (octets)\tSHA3-256");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Sha3_256);
	}

	#[test]
	fn header_sha3_256_end_tab() {
		let res = parse_header("Nom du document\tTaille (octets)\tSHA3-256\t");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Sha3_256);
	}

	#[test]
	fn header_blake2b() {
		let res = parse_header("Nom du document\tTaille (octets)\tBLAKE2b");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Blake2b);
	}

	#[test]
	fn header_blake2b_end_tab() {
		let res = parse_header("Nom du document\tTaille (octets)\tBLAKE2b\t");
		assert!(res.is_ok());
		let (_, hash_func) = res.unwrap();
		assert_eq!(hash_func, HashFunc::Blake2b);
	}

	#[test]
	fn header_empty() {
		let res = parse_header("");
		assert!(res.is_err());
	}

	#[test]
	fn line_simple() {
		let res = parse_line(
			"test_file.txt\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(file.get_size(), 42);
		assert_eq!(
			file.get_hash(),
			"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
		);
	}

	#[test]
	fn line_tab() {
		let res = parse_line(
			"test\tfile.txt\t\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test\tfile.txt\t"));
		assert_eq!(file.get_size(), 42);
		assert_eq!(
			file.get_hash(),
			"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
		);
	}

	#[test]
	fn line_simple_end_tab() {
		let res = parse_line(
			"test_file.txt\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\t",
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test_file.txt"));
		assert_eq!(file.get_size(), 42);
		assert_eq!(
			file.get_hash(),
			"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
		);
	}

	#[test]
	fn line_tab_end_tab() {
		let res = parse_line(
			"test\tfile.txt\t\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\t",
			HashFunc::Sha256,
		);
		assert!(res.is_ok());
		let (_, file) = res.unwrap();
		assert_eq!(file.get_relative_path(), Path::new("test\tfile.txt\t"));
		assert_eq!(file.get_size(), 42);
		assert_eq!(
			file.get_hash(),
			"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
		);
	}

	#[test]
	fn line_invalid_hash() {
		let res = parse_line(
			"test\tfile.txt\t\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852gggg",
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn line_invalid_size() {
		let res = parse_line(
			"test\tfile.txt\t\t0x2a\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn line_no_file_name() {
		let res = parse_line(
			"\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn line_no_file_name_end_tab() {
		let res = parse_line(
			"\t42\te3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\t",
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn line_no_size() {
		let res = parse_line(
			"test_file.txt\t\tb0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\t",
			HashFunc::Sha256,
		);
		assert!(res.is_err());
	}

	#[test]
	fn line_no_fingerprint() {
		let res = parse_line("test\tfile.txt\t\t42\t", HashFunc::Sha256);
		assert!(res.is_err());
	}

	#[test]
	fn line_empty() {
		let res = parse_line("", HashFunc::Sha256);
		assert!(res.is_err());
	}
}
