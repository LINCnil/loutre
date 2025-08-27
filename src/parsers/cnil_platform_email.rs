use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use msg_parser::Outlook;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1};
use nom::combinator::opt;
use nom::{IResult, Parser};
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const DEFAULT_HASH: HashFunc = HashFunc::Sha256;
const CNIL_V2_LST_BEGIN: &str = "Chaque empreinte";
const CNIL_V2_LST_END: &str = "Pour toute question";

pub fn cnil_platform_email_get_files_v2(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<HashedFileList, ()> {
	if let Ok(outlook) = Outlook::from_path(path) {
		let body: String = outlook.body.nfkc().collect();
		let mut files = HashedFileList::new();
		let mut is_lst = false;
		let mut last_file = None;
		for line in body.lines() {
			if line.is_empty() {
				continue;
			}
			if line.contains(CNIL_V2_LST_BEGIN) {
				is_lst = true;
				continue;
			}
			if line.contains(CNIL_V2_LST_END) {
				break;
			}
			if is_lst {
				match last_file {
					Some(name) => {
						let hash = clean_v2_hash(line)?;
						println!("Debug file:");
						println!(" - name: {name}");
						println!(" - hash: {hash}");
						let file = HashedFile::new(name, 0, hash, DEFAULT_HASH);
						files.insert_file(file);
						last_file = None;
					}
					None => {
						last_file = Some(clean_v2_name(line)?);
					}
				}
			}
		}
		if !files.is_empty() {
			return Ok(files);
		}
	}
	Err(())
}

fn clean_v2_name(input: &str) -> Result<String, ()> {
	let (input, _) = input.rsplit_once('(').ok_or(())?;
	let len = 0.max(input.len() - 1);
	let mut s = input.to_string();
	s.truncate(len);
	Ok(s)
}

fn clean_v2_hash(input: &str) -> Result<String, ()> {
	let (_, input) = input.split_once(' ').ok_or(())?;
	Ok(input.trim_end_matches(')').to_string())
}

pub fn cnil_platform_email_get_files_v1(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<HashedFileList, ()> {
	if let Ok(outlook) = Outlook::from_path(path) {
		let body: String = outlook.body.nfkc().collect();
		let mut files = HashedFileList::new();
		for line in body.lines() {
			if let Some(file) = parse_line(line) {
				files.insert_file(file);
			}
		}
		if !files.is_empty() {
			return Ok(files);
		}
	}
	Err(())
}

fn parse_line(line: &str) -> Option<HashedFile> {
	if let Some(line) = line.strip_prefix('*') {
		let line = line.trim_start();
		let line = rev_str(line);
		let (_, file) = parse_inner_line(&line).ok()?;
		return Some(file);
	}
	None
}

fn parse_inner_line(input: &str) -> IResult<&str, HashedFile> {
	let (input, _) = space0(input)?;
	let (input, _) = char(')')(input)?;
	let (input, hash) = alphanumeric1(input)?;
	let (input, _) = space1(input)?;
	let (input, _) = char(':')(input)?;
	let (input, _) = digit1(input)?;
	let (input, _) = char('-')(input)?;
	let (input, _) = alpha1(input)?;
	let (input, _) = space1(input)?;
	let (input, _) = char(',')(input)?;
	let (input, _) = alpha1(input)?;
	let (input, _) = space1(input)?;
	let (input, _) = digit1(input)?;
	let (input, _) = opt(char('.')).parse(input)?;
	let (input, _) = opt(digit1).parse(input)?;
	let (input, _) = char('(')(input)?;
	let (input, _) = space1(input)?;
	let hash = rev_str(hash);
	let path = PathBuf::from(rev_str(input));
	let file = HashedFile::new(path, 0, hash, DEFAULT_HASH);
	Ok((input, file))
}

fn rev_str(s: &str) -> String {
	s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
	use super::{cnil_platform_email_get_files_v2, DEFAULT_HASH};
	use crate::files::HashedFileList;
	use std::path::Path;

	const TEST_FILES_V2: &[(&str, &str)] = &[
		(
			"anssi-guide-admin_securisee_si_v3-0 - Copie - Copie (2) - Copie.pdf",
			"f7c943ec8788c797d4db829ddfca0919c543d3b046cec120e7433de2a3a25510",
		),
		(
			"Capture d'écran 2025-07-11 143340 - Copie - Copie - Copie.jpg",
			"90d0c07d696667bdca59365a3cf8ddc309f2f62dfa2a4e54b5117d82f0a0e3c3",
		),
		(
			"cijpfdp_2108b.exe",
			"8dd427942e8e31c574841b714286f9171dafe7b49dd781ec9b62ff0cc61cf1a4",
		),
	];

	#[test]
	fn test_v2() {
		let test_file_path = Path::new("src/parsers/tests_data/cnil_ar_v2.msg");
		let res = cnil_platform_email_get_files_v2(&test_file_path, DEFAULT_HASH);
		assert!(res.is_ok());
		let lst = res.unwrap();
		assert_eq!(lst.get_main_hashing_function(), DEFAULT_HASH);
		assert_eq!(lst.len(None), 3);
		for (ref_name, ref_hash) in TEST_FILES_V2 {
			assert!(
				contains_file(&lst, ref_name, ref_hash),
				"{ref_name}: file not found"
			);
		}
	}

	fn contains_file(lst: &HashedFileList, name: &str, hash: &str) -> bool {
		let ref_path = Path::new(name);
		for file in lst.get_files() {
			println!("test:");
			println!(" - {name}");
			println!(" - {}", file.get_relative_path().display());
			if file.get_hash() == hash && file.get_relative_path() == ref_path {
				return true;
			}
		}
		false
	}
}
