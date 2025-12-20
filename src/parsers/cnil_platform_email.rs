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
const CNIL_V3_PREFIX: &str = "*\t";

pub fn cnil_platform_email_get_files_v3(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<HashedFileList, ()> {
	tracing::debug!("Testing receipt type: CNILv3");
	if let Ok(outlook) = Outlook::from_path(path) {
		let body: String = outlook.body.nfkc().collect();
		let mut files = HashedFileList::new();
		for line in body.lines() {
			if line.starts_with(CNIL_V3_PREFIX) {
				tracing::debug!("line: {line}");
				if let Some(file) = parse_line_v3(line) {
					files.insert_file(file);
				}
			}
		}
		if !files.is_empty() {
			tracing::debug!("Found {} files.", files.len(None));
			return Ok(files);
		}
	}
	Err(())
}

fn parse_line_v3(line: &str) -> Option<HashedFile> {
	if let Some(line) = line.strip_prefix(CNIL_V3_PREFIX) {
		let line = rev_str(line);
		let (_, file) = parse_inner_line_v3(&line).ok()?;
		return Some(file);
	}
	None
}

fn parse_inner_line_v3(input: &str) -> IResult<&str, HashedFile> {
	let (input, _) = space0(input)?;
	let (input, _) = char(',')(input)?;
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
	let (input, _) = opt(char(',')).parse(input)?;
	let (input, _) = opt(digit1).parse(input)?;
	let (input, _) = char('(')(input)?;
	let (input, _) = space1(input)?;
	let hash = rev_str(hash);
	let path = PathBuf::from(rev_str(input));
	let file = HashedFile::new(path, 0, hash, DEFAULT_HASH);
	Ok((input, file))
}

pub fn cnil_platform_email_get_files_v2(
	path: &Path,
	_default_hash: HashFunc,
) -> Result<HashedFileList, ()> {
	tracing::debug!("Testing receipt type: CNILv2");
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
			tracing::debug!("Found {} files.", files.len(None));
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
	tracing::debug!("Testing receipt type: CNILv1");
	if let Ok(outlook) = Outlook::from_path(path) {
		let body: String = outlook.body.nfkc().collect();
		let mut files = HashedFileList::new();
		for line in body.lines() {
			if let Some(file) = parse_line_v1(line) {
				files.insert_file(file);
			}
		}
		if !files.is_empty() {
			tracing::debug!("Found {} files.", files.len(None));
			return Ok(files);
		}
	}
	Err(())
}

fn parse_line_v1(line: &str) -> Option<HashedFile> {
	if let Some(line) = line.strip_prefix('*') {
		let line = line.trim_start();
		let line = rev_str(line);
		let (_, file) = parse_inner_line_v1(&line).ok()?;
		return Some(file);
	}
	None
}

fn parse_inner_line_v1(input: &str) -> IResult<&str, HashedFile> {
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
	use super::{parse_line_v3, DEFAULT_HASH};
	use std::path::Path;

	const TEST_LINES_V3: &[(&str, &str, &str)] = &[
		("*	simple.txt (0,4 Ko, SHA-256: 0ea83f243ec71af1d50285617e8da3962a602a41266e8069d24de53b3c9d606c),", "simple.txt", "0ea83f243ec71af1d50285617e8da3962a602a41266e8069d24de53b3c9d606c"),
		("*	une espace.txt (0,4 Ko, SHA-256: daf394a0f43982e02ac31c78750e0fed82c5f495457967a2770da7ec263dd624),", "une espace.txt", "daf394a0f43982e02ac31c78750e0fed82c5f495457967a2770da7ec263dd624"),
		("*	 espaces en début de nom et accent.txt (0,6 Ko, SHA-256: 7bdc7dd8f1ade0b07d48a77940dab379ba685b63b0d5e11c4d6889ed7ae519cf),", " espaces en début de nom et accent.txt", "7bdc7dd8f1ade0b07d48a77940dab379ba685b63b0d5e11c4d6889ed7ae519cf"),
		("*	Deux  espaces.txt (0,7 Ko, SHA-256: 11cc874da1cc4bbdea97bc00cf299850c7dbaff60c42da2ea81d8892b03feb94),", "Deux  espaces.txt", "11cc874da1cc4bbdea97bc00cf299850c7dbaff60c42da2ea81d8892b03feb94"),
		("*	des accents è_é ÀøË—⇒Į.txt (0,6 Ko, SHA-256: fbff754afe96c6d70bbbb8d426d3fc325729d77020f88c0202d520f4114de23c),", "des accents è_é ÀøË—⇒Į.txt", "fbff754afe96c6d70bbbb8d426d3fc325729d77020f88c0202d520f4114de23c"),
		("*	Beaucoup    d' espaces   .txt (0,4 Ko, SHA-256: cbe462d43c025285f50abec2a6c86f261be2225b6663436193f60b0ec6cb5f90),", "Beaucoup    d' espaces   .txt", "cbe462d43c025285f50abec2a6c86f261be2225b6663436193f60b0ec6cb5f90"),
		("*	Ṱ̴͊e̴̛͚ṡ̴̘t̶͔͗ ̴̔ͅż̸̹a̵͙̽l̴̟̃g̷̡̾o̴͙͑.txt (0,5 Ko, SHA-256: b7d6a4679b21b01e66a07bc18f6d3b22cfbec0b795bf230c8e91ebd3a99d453f),", "Ṱ̴͊e̴̛͚ṡ̴̘t̶͔͗ ̴̔ͅż̸̹a̵͙̽l̴̟̃g̷̡̾o̴͙͑.txt", "b7d6a4679b21b01e66a07bc18f6d3b22cfbec0b795bf230c8e91ebd3a99d453f"),
	];

	#[test]
	fn test_v3() {
		for (line, file_name, hash) in TEST_LINES_V3 {
			let res = parse_line_v3(line);
			assert!(res.is_some());
			let f = res.unwrap();
			let path = Path::new(file_name);
			assert_eq!(f.get_hash_func(), DEFAULT_HASH);
			assert_eq!(f.get_hash(), *hash);
			assert_eq!(f.get_relative_path(), path);
		}
	}
}
