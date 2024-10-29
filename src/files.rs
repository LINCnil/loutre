use crate::events::ExternalEventSender;
use crate::hash::HashFunc;
use dioxus_logger::tracing::{error, info};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;

#[derive(Clone, Debug, Default)]
pub enum FileList {
	NonHashed(NonHashedFileList),
	Hashed(HashedFileList),
	#[default]
	None,
}

macro_rules! common_lst_impl {
	($file_type: ty) => {
		impl $file_type {
			pub fn get_base_dir(&self) -> &Path {
				self.base_dir.as_path()
			}
		}
	};
}

#[derive(Debug, Clone)]
pub struct NonHashedFileList {
	base_dir: PathBuf,
	files: HashMap<Vec<u8>, NonHashedFile>,
}

common_lst_impl!(NonHashedFileList);

impl NonHashedFileList {
	pub async fn from_dir<P: AsRef<Path>>(dir_path: P) -> io::Result<Self> {
		let dir_path = dir_path.as_ref().to_path_buf();
		let files = walkdir::WalkDir::new(&dir_path)
			.follow_links(false)
			.into_iter()
			.filter_map(|entry| match entry {
				Ok(entry) => {
					if entry.file_type().is_file() {
						match NonHashedFile::new(&dir_path, &entry.clone().into_path()) {
							Ok(file) => {
								let id = file.get_id();
								info!("File loaded: {}", file.relative_path.display());
								return Some((id, file));
							}
							Err(e) => {
								error!("{}: unable to read file: {e}", entry.into_path().display());
								return None;
							}
						}
					}
					None
				}
				Err(e) => {
					error!("Error while loading file: {e}");
					None
				}
			})
			.collect();
		Ok(Self {
			base_dir: dir_path,
			files,
		})
	}

	pub async fn hash(
		&self,
		hash_func: HashFunc,
		tx: ExternalEventSender,
	) -> io::Result<HashedFileList> {
		// TODO: use several threads with an LPT
		let handle_list: JoinSet<_> = self
			.files
			.iter()
			.map(|(k, f)| {
				let k = k.clone();
				let f = f.to_owned();
				let tx = tx.clone();
				tokio::spawn(async move {
					let file = f.hash(hash_func, tx).await?;
					Ok((k, file))
				})
			})
			.collect();
		let mut files = HashMap::with_capacity(self.files.len());
		for r in handle_list.join_all().await {
			let r: io::Result<_> = r?;
			let (k, f) = r?;
			files.insert(k, f);
		}
		Ok(HashedFileList {
			base_dir: self.base_dir.clone(),
			files,
		})
	}
}

#[derive(Debug, Clone)]
pub struct HashedFileList {
	base_dir: PathBuf,
	files: HashMap<Vec<u8>, HashedFile>,
}

common_lst_impl!(HashedFileList);

macro_rules! common_file_impl {
	($file_type: ty) => {
		impl $file_type {
			pub fn get_id(&self) -> Vec<u8> {
				[
					self.base_dir.as_os_str().as_encoded_bytes(),
					self.relative_path.as_os_str().as_encoded_bytes(),
				]
				.join(&0)
			}

			pub fn get_base_dir(&self) -> &Path {
				self.base_dir.as_path()
			}

			pub fn get_relative_path(&self) -> &Path {
				self.relative_path.as_path()
			}

			pub fn get_absolute_path(&self) -> io::Result<PathBuf> {
				let mut path = self.base_dir.clone();
				path.push(self.relative_path.clone());
				path.canonicalize()
			}
		}
	};
}

#[derive(Clone, Debug)]
pub struct NonHashedFile {
	base_dir: PathBuf,
	relative_path: PathBuf,
	size: u64,
}

common_file_impl!(NonHashedFile);

impl NonHashedFile {
	pub fn new<P: AsRef<Path>>(base_dir: P, path: P) -> io::Result<Self> {
		let base_dir = base_dir.as_ref();
		let path = path.as_ref();
		let relative_path = path.strip_prefix(base_dir).unwrap_or(path);
		let mut file = Self {
			base_dir: base_dir.to_path_buf(),
			relative_path: relative_path.to_path_buf(),
			size: 0,
		};
		file.size = file.get_absolute_path()?.metadata()?.len();
		Ok(file)
	}

	pub async fn hash(
		&self,
		hash_func: HashFunc,
		tx: ExternalEventSender,
	) -> io::Result<HashedFile> {
		Ok(HashedFile {
			base_dir: self.base_dir.clone(),
			relative_path: self.relative_path.clone(),
			size: self.size,
			hash: hash_func.hash_file(self.get_absolute_path()?, tx).await?,
			hash_func,
		})
	}
}

#[derive(Clone, Debug)]
pub struct HashedFile {
	base_dir: PathBuf,
	relative_path: PathBuf,
	size: u64,
	hash: String,
	hash_func: HashFunc,
}

common_file_impl!(HashedFile);

impl HashedFile {
	pub fn new<P, S>(base_dir: P, relative_path: P, size: u64, hash: S, hash_func: HashFunc) -> Self
	where
		P: AsRef<Path>,
		S: AsRef<str>,
	{
		Self {
			base_dir: base_dir.as_ref().to_path_buf(),
			relative_path: relative_path.as_ref().to_path_buf(),
			size,
			hash: hash.as_ref().into(),
			hash_func,
		}
	}
}
