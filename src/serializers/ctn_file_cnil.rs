use crate::files::HashedFileList;
use std::fs::File;
use std::io::{self, Write};

macro_rules! write_line {
	($file: ident, $name: expr, $size: expr, $hash: expr) => {
		let line = format!("{}\t{}\t{}\r\n", $name, $size, $hash);
		$file.write_all(line.as_bytes())?;
	};
}

pub fn ctn_file_cnil(ctn_file: &mut File, hashed_list: &HashedFileList) -> io::Result<()> {
	write_line!(
		ctn_file,
		"Nom du document",
		"Taille (octets)",
		hashed_list.get_main_hashing_function()
	);
	for file in hashed_list.get_files_no_base_dir() {
		write_line!(
			ctn_file,
			file.get_relative_path().display(),
			file.get_size(),
			file.get_hash()
		);
	}
	Ok(())
}
