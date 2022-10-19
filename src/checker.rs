use crate::email::Email;
use crate::file_list::FileList;
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use std::fmt::{self, Write};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

const MSG_INVALID_FILE_FORMAT: &str = "format du fichier invalide";
const ERR_DIFF_CALC_AR: &str = "Différences avec l’accusé de réception";
const ERR_DIFF_CALC_CTN: &str = "Différences avec le fichier";

macro_rules! load_differences {
	($set1: ident, $set2: ident, $err_base: ident, $name: expr, $err: ident) => {
		let mut diff: Vec<&File> = $set1.symmetric_difference(&$set2).collect();
		diff.sort();
		if !$name.is_empty() {
			let _ = writeln!($err, "{} {} :", $err_base, $name);
		} else {
			let _ = writeln!($err, "{}:", $err_base);
		};
		$err += &diff
			.iter()
			.filter(|f| $set2.iter().any(|e| e.path == f.path))
			.map(|f| format!(" - {}", f))
			.collect::<Vec<String>>()
			.join("\n");
	};
}

struct ContentFileError {
	msg: String,
}

impl ContentFileError {
	fn invalid_fmt() -> Self {
		MSG_INVALID_FILE_FORMAT.into()
	}
}

impl From<&str> for ContentFileError {
	fn from(error: &str) -> Self {
		ContentFileError {
			msg: error.to_string(),
		}
	}
}

impl From<std::num::ParseIntError> for ContentFileError {
	fn from(_error: std::num::ParseIntError) -> Self {
		ContentFileError::invalid_fmt()
	}
}

impl From<std::io::Error> for ContentFileError {
	fn from(error: std::io::Error) -> Self {
		ContentFileError {
			msg: error.to_string(),
		}
	}
}

#[derive(PartialEq, Eq, Hash)]
struct File {
	path: PathBuf,
	hash: String,
}

impl File {
	fn new(path: &Path, hash: &str) -> Self {
		File {
			path: normalize_path(path),
			hash: hash.to_string(),
		}
	}
}

impl From<&crate::file::File> for File {
	fn from(f: &crate::file::File) -> Self {
		File {
			path: normalize_path(&f.get_file_name()),
			hash: f
				.get_hash()
				.map(|e| e.to_owned())
				.unwrap_or_else(String::new),
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
	file_list: &FileList,
	content_file_name: &str,
	email: &Option<Email>,
) -> Result<(), String> {
	let content_file_set =
		load_content_file(file_list).map_err(|e| format!("{}: {}", content_file_name, e.msg))?;
	let calculated_set: HashSet<File> = file_list.iter_files().map(File::from).collect();
	let mut error_msg = String::new();
	if !content_file_set.is_subset(&calculated_set) {
		load_differences!(
			calculated_set,
			content_file_set,
			ERR_DIFF_CALC_CTN,
			content_file_name,
			error_msg
		);
	}
	if let Some(em_lst) = email {
		let email_set: HashSet<File> = em_lst.iter_files().map(File::from).collect();
		if !email_set.is_subset(&calculated_set) {
			if !error_msg.is_empty() {
				error_msg += "\n\n";
			}
			load_differences!(calculated_set, email_set, ERR_DIFF_CALC_AR, "", error_msg);
		}
	}
	if error_msg.is_empty() {
		Ok(())
	} else {
		Err(error_msg)
	}
}

fn load_content_file(file_list: &FileList) -> Result<HashSet<File>, ContentFileError> {
	let file = fs::File::open(file_list.get_content_file_path())?;
	let reader = BufReader::new(file);
	let mut lst = HashSet::new();
	let mut header = true;
	for line in reader.lines() {
		if header {
			header = false;
			continue;
		}
		let line = line?;
		let v: Vec<&str> = line.split('\t').collect();
		let nb_elems = v.len();
		if nb_elems == 3 || nb_elems == 4 {
			let mut path = *v.first().ok_or_else(ContentFileError::invalid_fmt)?;
			if !crate::CONTENT_FILE_PATH_PREFIX.is_empty() {
				path = match path.strip_prefix(crate::CONTENT_FILE_PATH_PREFIX) {
					Some(rp) => rp,
					None => path,
				};
			}
			let file_path = PathBuf::from(path);
			let file_hash = v
				.get(2)
				.ok_or_else(ContentFileError::invalid_fmt)?
				.to_string();
			lst.insert(File::new(&file_path, &file_hash));
		} else {
			return Err(ContentFileError::invalid_fmt());
		}
	}
	Ok(lst)
}

fn normalize_path(path: &Path) -> PathBuf {
	let mut ret = PathBuf::new();
	for cmp in path.components() {
		match cmp.as_os_str().to_str() {
			Some(s) => {
				let ns: String = s.nfkc().collect();
				ret.push(&ns);
			}
			None => {
				ret.push(cmp);
			}
		}
	}
	ret
}
