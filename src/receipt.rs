use crate::file::File;
use crate::hasher::HashFunc;
use crate::parsers::cnil_platform_email_get_files;
use std::fmt;
use std::path::{Path, PathBuf};
use std::slice::Iter;

const PARSERS: &[&dyn Fn(&Path, HashFunc) -> Result<(Vec<File>, HashFunc), ()>] =
	&[&cnil_platform_email_get_files];

pub struct Receipt {
	path: PathBuf,
	files: Vec<File>,
	hash_func: HashFunc,
}

impl Receipt {
	pub fn new(path: &Path, default_hash: HashFunc) -> Result<Self, ()> {
		let (files, hash_func) = get_files(path, default_hash)?;
		Ok(Self {
			path: path.to_owned(),
			files,
			hash_func,
		})
	}

	pub fn iter_files(&self) -> Iter<File> {
		self.files.iter()
	}

	pub fn get_hash_func(&self) -> HashFunc {
		self.hash_func
	}
}

impl fmt::Display for Receipt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.path.display())
	}
}

fn get_files(path: &Path, default_hash: HashFunc) -> Result<(Vec<File>, HashFunc), ()> {
	for parser in PARSERS {
		if let Ok((files, hash_func)) = parser(path, default_hash) {
			return Ok((files, hash_func));
		}
	}
	Err(())
}
