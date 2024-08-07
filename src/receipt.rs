use crate::file::File;
use crate::parsers::cnil_platform_email_get_files;
use std::fmt;
use std::path::{Path, PathBuf};
use std::slice::Iter;

pub struct Receipt {
	path: PathBuf,
	files: Vec<File>,
}

impl Receipt {
	pub fn new(path: &Path) -> Result<Self, ()> {
		Ok(Self {
			path: path.to_owned(),
			files: cnil_platform_email_get_files(path)?,
		})
	}

	pub fn iter_files(&self) -> Iter<File> {
		self.files.iter()
	}
}

impl fmt::Display for Receipt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.path.display())
	}
}
