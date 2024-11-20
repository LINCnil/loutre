use crate::files::HashedFile;
use dioxus_logger::tracing::warn;
use std::collections::HashSet;
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

pub fn check(
	calculated_set: &Vec<HashedFile>,
	reference_set: &Vec<HashedFile>,
	t: CheckType,
) -> CheckResult {
	let mut errors = HashSet::new();

	for ref_file in reference_set {
		// Get the canonical absolute path of the reference file.
		match ref_file.get_absolute_path() {
			Ok(ref_file_abs_path) => {
				// We have the canonical absolute path of the reference file.
				// Now, let's check if we can find it in the calculated set.
				match get_calc_file(calculated_set, ref_file_abs_path) {
					Ok(calc_file) => {
						if ref_file.get_hash() != calc_file.get_hash() {
							// The hashes from both files does not match.
							add_non_matching_file(&mut errors, ref_file, t);
						}
					}
					Err(_) => {
						// No matching file found in the calculated set.
						add_missing_file(&mut errors, ref_file, t);
					}
				};
			}
			Err(_) => {
				// Unable to get the canonical path: the file does not exists on disk.
				add_missing_file(&mut errors, ref_file, t);
			}
		};
	}

	// Return the result
	if errors.is_empty() {
		CheckResult::Ok
	} else {
		CheckResult::Error(errors.into_iter().collect())
	}
}

fn get_calc_file(
	calculated_set: &Vec<HashedFile>,
	ref_file_abs_path: PathBuf,
) -> Result<&HashedFile, ()> {
	for calc_file in calculated_set {
		if let Ok(calc_file_abs_path) = calc_file.get_absolute_path() {
			if calc_file_abs_path == ref_file_abs_path {
				return Ok(calc_file);
			}
		}
	}
	Err(())
}

#[inline]
fn add_missing_file(errors: &mut HashSet<CheckResultError>, file: &HashedFile, t: CheckType) {
	let path = file.get_relative_path().to_path_buf();
	let e = match t {
		CheckType::ContentFile => CheckResultError::ContentFileMissingFile(path),
		CheckType::Receipt => CheckResultError::ReceiptMissingFile(path),
	};
	warn!("{e}");
	errors.insert(e);
}

#[inline]
fn add_non_matching_file(errors: &mut HashSet<CheckResultError>, file: &HashedFile, t: CheckType) {
	let path = file.get_relative_path().to_path_buf();
	let e = match t {
		CheckType::ContentFile => CheckResultError::ContentFileNonMatchingFile(path),
		CheckType::Receipt => CheckResultError::ReceiptNonMatchingFile(path),
	};
	warn!("{e}");
	errors.insert(e);
}
