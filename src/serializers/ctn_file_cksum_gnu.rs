use crate::files::HashedFileList;
use std::fs::File;
use std::io::{self, Write};

pub fn ctn_file_cksum_gnu(ctn_file: &mut File, hashed_list: &HashedFileList) -> io::Result<()> {
	for file in hashed_list.get_files() {
		let line = format!(
			"{} *{}\r\n",
			file.get_hash(),
			file.get_relative_path().display(),
		);
		ctn_file.write_all(line.as_bytes())?;
	}
	Ok(())
}
