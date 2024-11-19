use crate::files::HashedFileList;
use crate::hash::HashFunc;
use crate::serializers::{ctn_file_cksum_bsd, ctn_file_cksum_gnu, ctn_file_cnil};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentFileFormat {
	#[default]
	#[serde(alias = "cksum-bsd")]
	CksumBsd,
	#[serde(alias = "cksum-gnu")]
	CksumGnu,
	Cnil,
}

impl fmt::Display for ContentFileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ctn_file_fmt = match &self {
			Self::CksumBsd => "Cksum (BSD)",
			Self::CksumGnu => "Cksum (GNU)",
			Self::Cnil => "Cnil",
		};
		write!(f, "{ctn_file_fmt}")
	}
}

impl ContentFileFormat {
	pub fn default_content_file_name(&self, hash_func: HashFunc) -> String {
		match self {
			Self::CksumBsd => format!("{}.txt", hash_func.to_string().to_lowercase()),
			Self::CksumGnu => format!("{}sums.txt", hash_func.to_string().to_lowercase()),
			Self::Cnil => String::from("contenu.txt"),
		}
	}

	pub fn write_content_file(
		&self,
		ctn_file: &mut File,
		hashed_list: &HashedFileList,
	) -> io::Result<()> {
		match self {
			Self::CksumBsd => ctn_file_cksum_bsd(ctn_file, hashed_list),
			Self::CksumGnu => ctn_file_cksum_gnu(ctn_file, hashed_list),
			Self::Cnil => ctn_file_cnil(ctn_file, hashed_list),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_content_file_name() {
		let tests = &[
			(ContentFileFormat::CksumBsd, HashFunc::Sha256, "sha256.txt"),
			(ContentFileFormat::CksumBsd, HashFunc::Sha512, "sha512.txt"),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Sha3_384,
				"sha3-384.txt",
			),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Blake2b,
				"blake2b.txt",
			),
			(ContentFileFormat::CksumBsd, HashFunc::Blake3, "blake3.txt"),
			(
				ContentFileFormat::CksumGnu,
				HashFunc::Sha256,
				"sha256sums.txt",
			),
			(
				ContentFileFormat::CksumGnu,
				HashFunc::Sha512,
				"sha512sums.txt",
			),
			(
				ContentFileFormat::CksumGnu,
				HashFunc::Sha3_384,
				"sha3-384sums.txt",
			),
			(
				ContentFileFormat::CksumGnu,
				HashFunc::Blake2b,
				"blake2bsums.txt",
			),
			(
				ContentFileFormat::CksumGnu,
				HashFunc::Blake3,
				"blake3sums.txt",
			),
			(ContentFileFormat::Cnil, HashFunc::Sha256, "contenu.txt"),
		];
		for (cff, hf, ref_name) in tests {
			let file_name = cff.default_content_file_name(*hf);
			assert_eq!(file_name, ref_name.to_string());
		}
	}
}
