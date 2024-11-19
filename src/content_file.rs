use crate::files::HashedFileList;
use crate::serializers::ctn_file_cnil;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ContentFileFormat {
	#[default]
	Cnil,
}

impl fmt::Display for ContentFileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ctn_file_fmt = match &self {
			Self::Cnil => "cnil",
		};
		write!(f, "{ctn_file_fmt}")
	}
}

impl ContentFileFormat {
	pub fn default_content_file_name(&self) -> String {
		match self {
			Self::Cnil => String::from("contenu.txt"),
		}
	}

	pub fn write_content_file(
		&self,
		ctn_file: &mut File,
		hashed_list: &HashedFileList,
	) -> io::Result<()> {
		match self {
			Self::Cnil => ctn_file_cnil(ctn_file, hashed_list),
		}
	}
}
