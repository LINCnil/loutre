use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use msg_parser::Outlook;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1};
use nom::combinator::opt;
use nom::{IResult, Parser};
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const DEFAULT_HASH: HashFunc = HashFunc::Sha256;

pub fn cnil_platform_email_get_files(
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
