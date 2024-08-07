use crate::file::File;
use crate::file_list::FileList;
use crate::hasher::HashFunc;
use crate::i18n::I18n;
use std::collections::hash_set::{HashSet, Iter};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct ContentFile {
	hash: HashFunc,
	files: HashSet<(PathBuf, String)>,
}

impl ContentFile {
	pub fn load(i18n: &I18n, file_list: &FileList) -> Result<Self, String> {
		let ctn_path = file_list.get_content_file_path();
		let file = fs::File::open(ctn_path).map_err(|e| e.to_string())?;
		let reader = BufReader::new(file);
		let mut lst = HashSet::new();
		let mut header = true;
		let mut hash = HashFunc::default();
		for line in reader.lines() {
			let line = line.map_err(|e| e.to_string())?;
			let v: Vec<&str> = line.split('\t').collect();
			let nb_elems = v.len();
			if nb_elems == 3 || nb_elems == 4 {
				if header {
					header = false;
					let hash_str = v
						.get(2)
						.ok_or_else(|| i18n.msg("msg_check_invalid_format"))?
						.trim();
					hash = HashFunc::parse(hash_str)
						.map_err(|_| i18n.msg("msg_check_invalid_format"))?;
					continue;
				}
				let mut path = *v
					.first()
					.ok_or_else(|| i18n.msg("msg_check_invalid_format"))?;
				// Although CONTENT_FILE_PATH_PREFIX is a const, its value may vary between
				// platforms. And no, checking the platform here is not the correct solution since
				// that would require to keep it updated with CONTENT_FILE_PATH_PREFIX, which I'm
				// 100% sure it will be forgotten.
				#[allow(clippy::const_is_empty)]
				if !crate::CONTENT_FILE_PATH_PREFIX.is_empty() {
					path = match path.strip_prefix(crate::CONTENT_FILE_PATH_PREFIX) {
						Some(rp) => rp,
						None => path,
					};
				}
				let file_path = PathBuf::from(path);
				let file_hash = v
					.get(2)
					.ok_or_else(|| i18n.msg("msg_check_invalid_format"))?
					.to_string();
				lst.insert((file_path, file_hash));
			} else {
				return Err(i18n.msg("msg_check_invalid_format"));
			}
		}
		Ok(Self { hash, files: lst })
	}

	pub fn get_hash_func(&self) -> HashFunc {
		self.hash
	}

	pub fn iter_files(&self) -> Iter<(PathBuf, String)> {
		self.files.iter()
	}

	pub fn write(
		i18n: &I18n,
		path: &PathBuf,
		files: &mut [File],
		hash: HashFunc,
	) -> io::Result<()> {
		files.sort_by(File::cmp_name);
		let mut content_file = fs::File::create(path)?;
		let header = format!(
			"{}\t{}\t{}\r\n",
			i18n.msg("content_file_header.name"),
			i18n.msg("content_file_header.size"),
			hash,
		);
		content_file.write_all(header.as_bytes())?;
		for f in files {
			content_file.write_all(&f.get_content_file_line())?;
		}
		Ok(())
	}

	pub fn len(&self) -> usize {
		self.files.len()
	}
}
