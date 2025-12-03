use crate::files::HashedFileList;
use crate::hash::HashFunc;
use crate::parsers::{
	cksum_bsd_get_files, cksum_gnu_get_files, cnil_content_file_get_files,
	cnil_platform_email_get_files_v1, cnil_platform_email_get_files_v2,
	cnil_platform_email_get_files_v3,
};
use std::fmt;
use std::path::{Path, PathBuf};

type ReceiptParser = dyn Fn(&Path, HashFunc) -> Result<HashedFileList, ()>;

const PARSERS: &[&ReceiptParser] = &[
	&cksum_bsd_get_files,
	&cksum_gnu_get_files,
	&cnil_content_file_get_files,
	&cnil_platform_email_get_files_v3,
	&cnil_platform_email_get_files_v2,
	&cnil_platform_email_get_files_v1,
];

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

	pub fn get_file_list(&self) -> &HashedFileList {
		&self.files
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
