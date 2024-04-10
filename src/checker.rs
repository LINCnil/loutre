use crate::content_file::ContentFile;
use crate::email::Email;
use crate::file_list::FileList;
use crate::i18n::{Attr, I18n};
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use std::fmt::{self, Write};
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

macro_rules! load_differences {
	($set1: ident, $set2: ident, $i18n: ident, $name: expr, $err: ident) => {
		let mut diff: Vec<&File> = $set1.symmetric_difference(&$set2).collect();
		diff.sort();
		let msg = if !$name.is_empty() {
			$i18n.fmt(
				"msg_err_diff_calc_ctn",
				&[("file_name", Attr::String($name.to_string()))],
			)
		} else {
			$i18n.msg("msg_err_diff_calc_ar")
		};
		let _ = writeln!($err, "{}", msg);
		$err += &diff
			.iter()
			.filter(|f| $set2.iter().any(|e| e.path == f.path))
			.map(|f| format!(" - {}", f))
			.collect::<Vec<String>>()
			.join("\n");
	};
}

enum ContentFileError {
	InvalidFormat,
	Other(String),
}

impl ContentFileError {
	fn disp(&self, i18n: &I18n) -> String {
		match self {
			ContentFileError::InvalidFormat => i18n.msg("msg_check_invalid_format"),
			ContentFileError::Other(msg) => msg.to_owned(),
		}
	}
}

impl From<&str> for ContentFileError {
	fn from(error: &str) -> Self {
		ContentFileError::Other(error.to_string())
	}
}

impl From<std::num::ParseIntError> for ContentFileError {
	fn from(_error: std::num::ParseIntError) -> Self {
		ContentFileError::InvalidFormat
	}
}

impl From<std::io::Error> for ContentFileError {
	fn from(error: std::io::Error) -> Self {
		ContentFileError::Other(error.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
) -> Result<Vec<PathBuf>, String> {
	let content_file_set = load_content_file(i18n, file_list).map_err(|e| {
		i18n.fmt(
			"error_desc",
			&[
				("error", Attr::String(content_file_name.to_string())),
				("description", Attr::String(e.disp(i18n))),
			],
		)
	})?;
	let calculated_set: HashSet<File> = file_list.iter_files().map(File::from).collect();
	let mut error_msg = String::new();
	if !content_file_set.is_subset(&calculated_set) {
		load_differences!(
			calculated_set,
			content_file_set,
			i18n,
			content_file_name,
			error_msg
		);
	}
	if let Some(em_lst) = email {
		// Dans les courriers électroniques, il est possible que les séries de
		// plusieurs espaces soient réduites à une seule espace. Si une telle
		// modification a lieu dans un AR, il sera impossible de trouver le fichier
		// correspondant sur le système et une erreur sera affichée. Du fait que ce
		// comportement ne soit pas garanti, il faut absolument tester les deux
		// variantes et ne relever une erreur que si les deux échouent.
		let email_set: HashSet<File> = em_lst.iter_files().map(File::from).collect();
		let fixed_names_set: HashSet<File> = calculated_set.iter().map(File::fix_name).collect();
		if !email_set.is_subset(&calculated_set) && !email_set.is_subset(&fixed_names_set) {
			if !error_msg.is_empty() {
				error_msg += "\n\n";
			}
			load_differences!(calculated_set, email_set, i18n, "", error_msg);
		}
	}
	if error_msg.is_empty() {
		let paths_on_disk: HashSet<PathBuf> =
			calculated_set.iter().map(|e| e.path.to_owned()).collect();
		let paths_in_ctn_file: HashSet<PathBuf> =
			content_file_set.iter().map(|e| e.path.to_owned()).collect();
		let ignored_files: Vec<PathBuf> = paths_on_disk
			.difference(&paths_in_ctn_file)
			.map(|e| e.to_owned())
			.collect();
		Ok(ignored_files)
	} else {
		Err(error_msg)
	}
}

fn load_content_file(i18n: &I18n, file_list: &FileList) -> Result<HashSet<File>, ContentFileError> {
	let ctn_file = ContentFile::load(i18n, file_list).map_err(ContentFileError::Other)?;
	let mut lst = HashSet::new();
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
