use crate::files::{HashedFile, HashedFileList};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum CheckType {
	ContentFile,
	Receipt,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum CheckResultError {
	ContentFileParseError,
	ContentFileMissingFile(PathBuf),
	ContentFileNonMatchingFile(PathBuf),
	ReceiptMissingFile(PathBuf),
	ReceiptNonMatchingFile(PathBuf),
}

impl fmt::Display for CheckResultError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ctn_file_fmt = match &self {
			Self::ContentFileParseError => "content file: parse error".to_string(),
			Self::ContentFileMissingFile(p) => {
				format!("content file: missing file: {}", p.display())
			}
			Self::ContentFileNonMatchingFile(p) => {
				format!("content file: non matching file: {}", p.display())
			}
			Self::ReceiptMissingFile(p) => format!("receipt: missing file: {}", p.display()),
			Self::ReceiptNonMatchingFile(p) => {
				format!("receipt: non matching file: {}", p.display())
			}
		};
		write!(f, "{ctn_file_fmt}")
	}
}

#[derive(Debug, Clone)]
pub enum CheckResult {
	Error(Vec<CheckResultError>),
	Ok,
	None,
}

impl CheckResult {
	pub fn is_err(&self) -> bool {
		matches!(self, Self::Error(_))
	}

	pub fn is_ok(&self) -> bool {
		matches!(self, Self::Ok)
	}
}

macro_rules! populate_map {
	($dest_map: ident, $from_lst: ident, $base_dir: ident, $errors: ident, $t: ident, $add_err: expr) => {
		for ref_file in $from_lst.get_files() {
			let hash = ref_file.get_hash();
			let rel_path = ref_file.get_relative_path();
			let ref_file =
				HashedFile::new_base_dir($base_dir, rel_path, 0, hash, ref_file.get_hash_func());
			match ref_file.get_absolute_path() {
				Ok(absolute_path) => {
					$dest_map.insert(absolute_path, ref_file);
				}
				Err(_) => {
					if $add_err {
						add_missing_file(&mut $errors, &ref_file, $t);
					}
				}
			};
		}
	};
}

pub fn check(
	calculated_fl: &HashedFileList,
	reference_fl: &HashedFileList,
	t: CheckType,
) -> CheckResult {
	tracing::info!("Starting fingerprint check");
	let mut errors = HashSet::new();
	let base_dir = calculated_fl.get_base_dir();

	let mut reference_map = HashMap::with_capacity(reference_fl.len(None));
	populate_map!(reference_map, reference_fl, base_dir, errors, t, true);
	let mut calculated_map = HashMap::with_capacity(calculated_fl.len(None));
	populate_map!(calculated_map, calculated_fl, base_dir, errors, t, false);

	for (path, ref_file) in reference_map.iter() {
		match calculated_map.get(path) {
			Some(calc_file) => {
				if calc_file.get_hash() != ref_file.get_hash() {
					add_non_matching_file(&mut errors, ref_file, t);
				}
			}
			None => {
				add_missing_file(&mut errors, ref_file, t);
			}
		}
	}

	if errors.is_empty() {
		tracing::info!("Fingerprint check done: ok");
		CheckResult::Ok
	} else {
		tracing::warn!("Fingerprint check done: {} errors", errors.len());
		CheckResult::Error(errors.into_iter().collect())
	}
}

#[inline]
fn add_missing_file(errors: &mut HashSet<CheckResultError>, file: &HashedFile, t: CheckType) {
	let path = file.get_relative_path().to_path_buf();
	let e = match t {
		CheckType::ContentFile => CheckResultError::ContentFileMissingFile(path),
		CheckType::Receipt => CheckResultError::ReceiptMissingFile(path),
	};
	tracing::warn!("{e}");
	errors.insert(e);
}

#[inline]
fn add_non_matching_file(errors: &mut HashSet<CheckResultError>, file: &HashedFile, t: CheckType) {
	let path = file.get_relative_path().to_path_buf();
	let e = match t {
		CheckType::ContentFile => CheckResultError::ContentFileNonMatchingFile(path),
		CheckType::Receipt => CheckResultError::ReceiptNonMatchingFile(path),
	};
	tracing::warn!("{e}");
	errors.insert(e);
}
