use crate::analyse_hash::analyse_hash;
use crate::file::File;
use crate::hasher::HashFunc;
use msg_parser::Outlook;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1};
use nom::combinator::opt;
use nom::IResult;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const DEFAULT_HASH: HashFunc = HashFunc::Sha256;

macro_rules! ret_not_empty {
	($lst: ident, $hash: ident) => {
		if $lst.is_empty() {
			Err(())
		} else {
			Ok(($lst, $hash))
		}
	};
}

pub fn cnil_platform_email_get_files(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<(Vec<File>, HashFunc), ()> {
	if let Ok(outlook) = Outlook::from_path(path) {
		let body: String = outlook.body.nfkc().collect();
		let files = parse_txt(&body);
		let hash = match files.first() {
			Some(f) => match f.get_hash() {
				Some(h) => analyse_hash(h, DEFAULT_HASH),
				None => DEFAULT_HASH,
			},
			None => DEFAULT_HASH,
		};
		return ret_not_empty!(files, hash);
	}
	Err(())
}

fn parse_txt(content: &str) -> Vec<File> {
	let mut lst = Vec::new();
	for line in content.lines() {
		if let Some(l) = parse_line(line) {
			lst.push(l);
		}
	}
	lst
}

fn parse_line(line: &str) -> Option<File> {
	if let Some(line) = line.strip_prefix('*') {
		let line = line.trim_start();
		let line = rev_str(line);
		let (_, file) = parse_inner_line(&line).ok()?;
		return Some(file);
	}
	None
}

fn parse_inner_line(input: &str) -> IResult<&str, File> {
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
	let (input, _) = opt(char('.'))(input)?;
	let (input, _) = opt(digit1)(input)?;
	let (input, _) = char('(')(input)?;
	let (input, _) = space1(input)?;
	let hash = rev_str(hash);
	let path = PathBuf::from(rev_str(input));
	Ok((input, File::create_dummy(&path, &PathBuf::new(), 0, &hash)))
}

fn rev_str(s: &str) -> String {
	s.chars().rev().collect()
}
