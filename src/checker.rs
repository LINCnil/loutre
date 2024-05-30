use crate::content_file::ContentFile;
use crate::email::Email;
use crate::file_list::FileList;
use crate::i18n::{Attr, I18n};
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Default)]
pub struct CheckErrors {
	invalid_ctn_file: Vec<File>,
	invalid_email: Vec<File>,
	missing_ctn_file: Vec<File>,
	missing_email: Vec<File>,
}

impl CheckErrors {
	pub fn is_empty(&self) -> bool {
		self.invalid_ctn_file.is_empty()
			&& self.invalid_email.is_empty()
			&& self.missing_ctn_file.is_empty()
			&& self.missing_email.is_empty()
	}
}

#[derive(Debug)]
pub enum CheckResult {
	CheckErrors(CheckErrors),
	OtherError(String),
	Success(Vec<PathBuf>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct File {
	path: PathBuf,
	hash: String,
}

impl File {
	fn new(path: &Path, hash: &str) -> Self {
		Self {
			path: normalize_path(path, false),
			hash: hash.to_string(),
		}
	}

	fn fix_name(file: &Self) -> Self {
		Self {
			path: normalize_path(&file.path, true),
			hash: file.hash.to_owned(),
		}
	}
}

impl From<&crate::file::File> for File {
	fn from(f: &crate::file::File) -> Self {
		File {
			path: normalize_path(&f.get_file_name(), false),
			hash: f.get_hash().map(|e| e.to_owned()).unwrap_or_default(),
		}
	}
}

impl Ord for File {
	fn cmp(&self, other: &Self) -> Ordering {
		self.path.cmp(&other.path)
	}
}

impl PartialOrd for File {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl fmt::Display for File {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.hash, self.path.display(),)
	}
}

pub fn check_files(
	i18n: &I18n,
	file_list: &FileList,
	content_file_name: &str,
	email: &Option<Email>,
) -> CheckResult {
	match load_content_file(i18n, file_list) {
		Ok(content_file_set) => {
			let calculated_set: HashSet<File> = file_list.iter_files().map(File::from).collect();
			let email_set = email
				.as_ref()
				.map(|lst| lst.iter_files().map(File::from).collect::<HashSet<File>>());
			let errors = load_errors(&calculated_set, &content_file_set, &email_set);
			if errors.is_empty() {
				let paths_on_disk: HashSet<PathBuf> =
					calculated_set.iter().map(|e| e.path.to_owned()).collect();
				let paths_in_ctn_file: HashSet<PathBuf> =
					content_file_set.iter().map(|e| e.path.to_owned()).collect();
				let ignored_files: Vec<PathBuf> = paths_on_disk
					.difference(&paths_in_ctn_file)
					.map(|e| e.to_owned())
					.collect();
				CheckResult::Success(ignored_files)
			} else {
				CheckResult::CheckErrors(errors)
			}
		}
		Err(e) => CheckResult::OtherError(i18n.fmt(
			"error_desc",
			&[
				("error", Attr::String(content_file_name.to_string())),
				("description", Attr::String(e)),
			],
		)),
	}
}

fn loose_check_path(p1: &File, p2: &File) -> bool {
	(p1.path == p2.path) || (File::fix_name(p1).path == File::fix_name(p2).path)
}

fn load_errors(
	calculated_set: &HashSet<File>,
	content_file_set: &HashSet<File>,
	email_set_opt: &Option<HashSet<File>>,
) -> CheckErrors {
	let invalid_ctn_file = content_file_set
		.iter()
		.filter_map(|f| {
			if calculated_set
				.iter()
				.any(|e| loose_check_path(e, f) && e.hash != f.hash)
			{
				Some(f.clone())
			} else {
				None
			}
		})
		.collect();
	let invalid_email = match email_set_opt {
		Some(email_set) => email_set
			.iter()
			.filter_map(|f| {
				if calculated_set
					.iter()
					.any(|e| loose_check_path(e, f) && e.hash != f.hash)
				{
					Some(f.clone())
				} else {
					None
				}
			})
			.collect(),
		None => Vec::new(),
	};
	let missing_ctn_file = content_file_set
		.iter()
		.filter_map(|f| {
			if !calculated_set.iter().any(|e| loose_check_path(e, f)) {
				Some(f.clone())
			} else {
				None
			}
		})
		.collect();
	let missing_email = match email_set_opt {
		Some(email_set) => email_set
			.iter()
			.filter_map(|f| {
				if !calculated_set.iter().any(|e| e.path == f.path) {
					Some(f.clone())
				} else {
					None
				}
			})
			.collect(),
		None => Vec::new(),
	};

	CheckErrors {
		invalid_ctn_file,
		invalid_email,
		missing_ctn_file,
		missing_email,
	}
}

fn load_content_file(i18n: &I18n, file_list: &FileList) -> Result<HashSet<File>, String> {
	let ctn_file = ContentFile::load(i18n, file_list)?;
	let mut lst = HashSet::with_capacity(ctn_file.len());
	for (path, hash) in ctn_file.iter_files() {
		lst.insert(File::new(path, hash));
	}
	Ok(lst)
}

fn normalize_path(path: &Path, hard_fixes: bool) -> PathBuf {
	let mut ret = PathBuf::new();
	for cmp in path.components() {
		match cmp.as_os_str().to_str() {
			Some(s) => {
				let mut ns: String = s.nfkc().collect();
				if hard_fixes {
					collapse_spaces(&mut ns);
				}
				ret.push(&ns);
			}
			None => {
				ret.push(cmp);
			}
		}
	}
	ret
}

fn collapse_spaces(s: &mut String) {
	loop {
		*s = s.replace("  ", " ");
		if !s.contains("  ") {
			return;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_file_set_ref() -> HashSet<File> {
		HashSet::from([
			File {
				path: "file 01".into(),
				hash: "fb27331cd482123d92e6cf2b32fad64faa9ff8b15e91329b073762f98a24f5e7".into(),
			},
			File {
				path: "file 02".into(),
				hash: "e3d0e1c37da50c4f422d6dc6071d2a75accf8bcf727a4876e6698c360c5fb5c9".into(),
			},
			File {
				path: "file 03".into(),
				hash: "e6fd3b6f84f17e3ab73db9d87c2fc4e6b64f2a76e74a02dc30204e5c56093cc4".into(),
			},
			File {
				path: "file 04".into(),
				hash: "707cd5d25ad6a03b9a791e092fec6b0e5af981552549adce38ce3733bdc0687a".into(),
			},
			File {
				path: "file 05".into(),
				hash: "5fc3764a9f873eb9cd94bbb7ac459f6bc92f9a26508466ad57f93ac84174dcfd".into(),
			},
			File {
				path: "file 06".into(),
				hash: "522a977bf95bf2a0393dc352f7492271c080e51d55d18a5a16a4bda548371117".into(),
			},
			File {
				path: "file 07".into(),
				hash: "7301d0ce7edfa3b3b4c16aeb1b9f8b1994ded5b62b5dacb299e062da59a01763".into(),
			},
			File {
				path: "file 08".into(),
				hash: "d92f2a13c95cf8e58e9d5bdea4fcba4fac9a7e6169dec0dcf69bbbf91527f1bb".into(),
			},
			File {
				path: "file 09".into(),
				hash: "de6bb72a6b3e67bc7e6417c8df9775ad67ecd85fd7f32a37db35ed89a6aafa40".into(),
			},
			File {
				path: "file 10".into(),
				hash: "7cff2aeeb1cd9982b9e8ed2cfb2428d97a2dc6ed13faf01d509fdeada9ebf14e".into(),
			},
		])
	}

	fn get_file_set_err_missing() -> HashSet<File> {
		HashSet::from([
			File {
				path: "file 01".into(),
				hash: "fb27331cd482123d92e6cf2b32fad64faa9ff8b15e91329b073762f98a24f5e7".into(),
			},
			File {
				path: "file 02".into(),
				hash: "e3d0e1c37da50c4f422d6dc6071d2a75accf8bcf727a4876e6698c360c5fb5c9".into(),
			},
			File {
				path: "file 03".into(),
				hash: "e6fd3b6f84f17e3ab73db9d87c2fc4e6b64f2a76e74a02dc30204e5c56093cc4".into(),
			},
			File {
				path: "file 05".into(),
				hash: "5fc3764a9f873eb9cd94bbb7ac459f6bc92f9a26508466ad57f93ac84174dcfd".into(),
			},
			File {
				path: "file 06".into(),
				hash: "522a977bf95bf2a0393dc352f7492271c080e51d55d18a5a16a4bda548371117".into(),
			},
			File {
				path: "file 07".into(),
				hash: "7301d0ce7edfa3b3b4c16aeb1b9f8b1994ded5b62b5dacb299e062da59a01763".into(),
			},
			File {
				path: "file 08".into(),
				hash: "d92f2a13c95cf8e58e9d5bdea4fcba4fac9a7e6169dec0dcf69bbbf91527f1bb".into(),
			},
			File {
				path: "file 09".into(),
				hash: "de6bb72a6b3e67bc7e6417c8df9775ad67ecd85fd7f32a37db35ed89a6aafa40".into(),
			},
		])
	}

	fn get_file_set_err_invalid() -> HashSet<File> {
		HashSet::from([
			File {
				path: "file 01".into(),
				hash: "fb27331cd482123d92e6cf2b32fad64faa9ff8b15e91329b073762f98a24f5e7".into(),
			},
			File {
				path: "file 02".into(),
				hash: "e3d0e1c37da50c4f422d6dc6071d2a75accf8bcf727a4876e6698c360c5fb5c9".into(),
			},
			File {
				path: "file 03".into(),
				hash: "0b378963d327dcedaf5130f47dc4e0f7640750a4f3a3cd1cf5bd8dd1639a8951".into(),
			},
			File {
				path: "file 04".into(),
				hash: "707cd5d25ad6a03b9a791e092fec6b0e5af981552549adce38ce3733bdc0687a".into(),
			},
			File {
				path: "file 05".into(),
				hash: "5fc3764a9f873eb9cd94bbb7ac459f6bc92f9a26508466ad57f93ac84174dcfd".into(),
			},
			File {
				path: "file 06".into(),
				hash: "522a977bf95bf2a0393dc352f7492271c080e51d55d18a5a16a4bda548371117".into(),
			},
			File {
				path: "file 07".into(),
				hash: "a36742d958e53b51f708a5cc1d2a23774855ff959c8a1ad2495a9898171c0814".into(),
			},
			File {
				path: "file 08".into(),
				hash: "d92f2a13c95cf8e58e9d5bdea4fcba4fac9a7e6169dec0dcf69bbbf91527f1bb".into(),
			},
			File {
				path: "file 09".into(),
				hash: "5862de50fc21484e9645f467149395610cd840d73376334249a463d7a30ecf61".into(),
			},
			File {
				path: "file 10".into(),
				hash: "7cff2aeeb1cd9982b9e8ed2cfb2428d97a2dc6ed13faf01d509fdeada9ebf14e".into(),
			},
		])
	}

	fn get_file_set_err_both() -> HashSet<File> {
		HashSet::from([
			File {
				path: "file 02".into(),
				hash: "151a052cf47822f55afff0bed371a657449f9bb7cd810cc4e02b34dcafe08b53".into(),
			},
			File {
				path: "file 03".into(),
				hash: "0b378963d327dcedaf5130f47dc4e0f7640750a4f3a3cd1cf5bd8dd1639a8951".into(),
			},
			File {
				path: "file 05".into(),
				hash: "5fc3764a9f873eb9cd94bbb7ac459f6bc92f9a26508466ad57f93ac84174dcfd".into(),
			},
			File {
				path: "file 06".into(),
				hash: "522a977bf95bf2a0393dc352f7492271c080e51d55d18a5a16a4bda548371117".into(),
			},
			File {
				path: "file 07".into(),
				hash: "a36742d958e53b51f708a5cc1d2a23774855ff959c8a1ad2495a9898171c0814".into(),
			},
			File {
				path: "file 09".into(),
				hash: "5862de50fc21484e9645f467149395610cd840d73376334249a463d7a30ecf61".into(),
			},
			File {
				path: "file 10".into(),
				hash: "7cff2aeeb1cd9982b9e8ed2cfb2428d97a2dc6ed13faf01d509fdeada9ebf14e".into(),
			},
		])
	}

	#[test]
	fn test_ok() {
		let calculated_set = get_file_set_ref();
		let content_file_set = get_file_set_ref();
		let email_set = None;
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(res.is_empty(), "Unexpected errors: {:?}", res);
	}

	#[test]
	fn test_ok_email() {
		let calculated_set = get_file_set_ref();
		let content_file_set = get_file_set_ref();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(res.is_empty(), "Unexpected errors: {:?}", res);
	}

	#[test]
	fn test_ok_extra_files() {
		let calculated_set = get_file_set_ref();
		let content_file_set = get_file_set_err_missing();
		let email_set = Some(get_file_set_err_missing());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(res.is_empty(), "Unexpected errors: {:?}", res);
	}

	#[test]
	fn test_ok_charset() {
		let calculated_set = HashSet::from([File {
			path: "Test  file".into(),
			hash: "0e07a44042407a54075710cf2be8a8a8fbe2bc360b1ce7e03ce0acf85f304827".into(),
		}]);
		let content_file_set = HashSet::from([File {
			path: "Test file".into(),
			hash: "0e07a44042407a54075710cf2be8a8a8fbe2bc360b1ce7e03ce0acf85f304827".into(),
		}]);
		let email_set = None;
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(res.is_empty(), "Unexpected errors: {:?}", res);
	}

	#[test]
	fn test_err_missing() {
		let calculated_set = get_file_set_err_missing();
		let content_file_set = get_file_set_ref();
		let email_set = None;
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert!(res.invalid_ctn_file.is_empty());
		assert!(res.invalid_email.is_empty());
		assert_eq!(res.missing_ctn_file.len(), 2);
		assert!(res.missing_email.is_empty());
	}

	#[test]
	fn test_err_missing_email() {
		let calculated_set = get_file_set_err_missing();
		let content_file_set = get_file_set_ref();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert!(res.invalid_ctn_file.is_empty());
		assert!(res.invalid_email.is_empty());
		assert_eq!(res.missing_ctn_file.len(), 2);
		assert_eq!(res.missing_email.len(), 2);
	}

	#[test]
	fn test_err_missing_email_only() {
		let calculated_set = get_file_set_err_missing();
		let content_file_set = get_file_set_err_missing();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert!(res.invalid_ctn_file.is_empty());
		assert!(res.invalid_email.is_empty());
		assert!(res.missing_ctn_file.is_empty());
		assert_eq!(res.missing_email.len(), 2);
	}

	#[test]
	fn test_err_invalid() {
		let calculated_set = get_file_set_err_invalid();
		let content_file_set = get_file_set_ref();
		let email_set = None;
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert_eq!(res.invalid_ctn_file.len(), 3);
		assert!(res.invalid_email.is_empty());
		assert!(res.missing_ctn_file.is_empty());
		assert!(res.missing_email.is_empty());
	}

	#[test]
	fn test_err_invalid_email() {
		let calculated_set = get_file_set_err_invalid();
		let content_file_set = get_file_set_ref();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert_eq!(res.invalid_ctn_file.len(), 3);
		assert_eq!(res.invalid_email.len(), 3);
		assert!(res.missing_ctn_file.is_empty());
		assert!(res.missing_email.is_empty());
	}

	#[test]
	fn test_err_invalid_email_only() {
		let calculated_set = get_file_set_err_invalid();
		let content_file_set = get_file_set_err_invalid();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert!(res.invalid_ctn_file.is_empty());
		assert_eq!(res.invalid_email.len(), 3);
		assert!(res.missing_ctn_file.is_empty());
		assert!(res.missing_email.is_empty());
	}

	#[test]
	fn test_err_both() {
		let calculated_set = get_file_set_err_both();
		let content_file_set = get_file_set_ref();
		let email_set = None;
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert_eq!(res.invalid_ctn_file.len(), 4);
		assert!(res.invalid_email.is_empty());
		assert_eq!(res.missing_ctn_file.len(), 3);
		assert!(res.missing_email.is_empty());
	}

	#[test]
	fn test_err_both_email() {
		let calculated_set = get_file_set_err_both();
		let content_file_set = get_file_set_ref();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert_eq!(res.invalid_ctn_file.len(), 4);
		assert_eq!(res.invalid_email.len(), 4);
		assert_eq!(res.missing_ctn_file.len(), 3);
		assert_eq!(res.missing_email.len(), 3);
	}

	#[test]
	fn test_err_both_email_only() {
		let calculated_set = get_file_set_err_both();
		let content_file_set = get_file_set_err_both();
		let email_set = Some(get_file_set_ref());
		let res = load_errors(&calculated_set, &content_file_set, &email_set);
		assert!(!res.is_empty());
		assert!(res.invalid_ctn_file.is_empty());
		assert_eq!(res.invalid_email.len(), 4);
		assert!(res.missing_ctn_file.is_empty());
		assert_eq!(res.missing_email.len(), 3);
	}
}
