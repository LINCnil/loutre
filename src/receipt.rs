use crate::files::{HashedFile, HashedFileList};
use crate::hash::HashFunc;
use crate::parsers::{cksum_gnu_get_files, cnil_platform_email_get_files};
use std::fmt;
use std::path::{Path, PathBuf};

type ReceiptParser = dyn Fn(&Path, HashFunc) -> Result<HashedFileList, ()>;

const PARSERS: &[&ReceiptParser] = &[&cksum_gnu_get_files, &cnil_platform_email_get_files];

#[derive(Clone, Debug)]
pub struct Receipt {
	path: PathBuf,
	files: HashedFileList,
}

impl Receipt {
	pub fn new(path: &Path, default_hash: HashFunc) -> Result<Self, ()> {
		let files = get_files(path, default_hash)?;
		Ok(Self {
			path: path.to_owned(),
			files,
		})
	}

	pub fn get_files(&self, base_dir: &Path) -> Vec<HashedFile> {
		self.files.get_files(base_dir)
	}

	pub fn get_main_hashing_function(&self) -> HashFunc {
		self.files.get_main_hashing_function()
	}
}

impl fmt::Display for Receipt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.path.display())
	}
}

fn get_files(path: &Path, default_hash: HashFunc) -> Result<HashedFileList, ()> {
	for parser in PARSERS {
		if let Ok(files) = parser(path, default_hash) {
			return Ok(files);
		}
	}
	Err(())
}
