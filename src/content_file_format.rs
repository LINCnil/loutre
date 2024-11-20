use crate::files::HashedFileList;
use crate::hash::HashFunc;
use crate::serializers::{ctn_file_cksum_bsd, ctn_file_cksum_gnu, ctn_file_cnil};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Default, EnumIter, Deserialize, PartialEq, Serialize)]
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
			Self::Cnil => "CNIL",
		};
		write!(f, "{ctn_file_fmt}")
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseContentFileFormatError;

impl std::str::FromStr for ContentFileFormat {
	type Err = ParseContentFileFormatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"cksum-bsd" => Ok(Self::CksumBsd),
			"cksum-gnu" => Ok(Self::CksumGnu),
			"cnil" => Ok(Self::Cnil),
			_ => Err(ParseContentFileFormatError),
		}
	}
}

impl ContentFileFormat {
	pub fn get_value(&self) -> String {
		match self {
			Self::CksumBsd => "cksum-bsd",
			Self::CksumGnu => "cksum-gnu",
			Self::Cnil => "cnil",
		}
		.to_string()
	}

	pub fn default_content_file_name(&self, hash_func: HashFunc) -> String {
		match self {
			Self::CksumBsd => format!("CHECKSUM.{}.txt", hash_func),
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
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Sha256,
				"CHECKSUM.SHA256.txt",
			),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Sha512,
				"CHECKSUM.SHA512.txt",
			),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Sha3_384,
				"CHECKSUM.SHA3-384.txt",
			),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Blake2b,
				"CHECKSUM.BLAKE2b.txt",
			),
			(
				ContentFileFormat::CksumBsd,
				HashFunc::Blake3,
				"CHECKSUM.BLAKE3.txt",
			),
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
