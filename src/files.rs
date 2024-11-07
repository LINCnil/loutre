use crate::events::ExternalEventSender;
use crate::hash::HashFunc;
use dioxus_logger::tracing::{error, info};
use std::collections::{HashMap, HashSet};
use std::io;
#[cfg(windows)]
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;
use uuid::Uuid;

// Microsoft Windows File Attribute Constants
// https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
#[cfg(windows)]
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
#[cfg(windows)]
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;

#[derive(Clone, Debug, Default)]
pub enum FileList {
	NonHashed(NonHashedFileList),
	Hashed(HashedFileList),
	#[default]
	None,
}

impl FileList {
	pub fn get_id(&self) -> String {
		match self {
			Self::NonHashed(lst) => lst.get_id(),
			Self::Hashed(lst) => lst.get_id(),
			Self::None => String::new(),
		}
	}

	pub fn has_empty_files(&self) -> bool {
		match self {
			Self::NonHashed(lst) => !lst.empty_files.is_empty(),
			Self::Hashed(lst) => !lst.empty_files.is_empty(),
			Self::None => false,
		}
	}

	pub fn has_excluded_files(&self) -> bool {
		match self {
			Self::NonHashed(lst) => !lst.excluded_files.is_empty(),
			Self::Hashed(_) | Self::None => false,
		}
	}

	pub fn has_duplicated_files(&self) -> bool {
		match self {
			Self::Hashed(lst) => !lst.duplicated_files.is_empty(),
			Self::NonHashed(_) | Self::None => false,
		}
	}

	pub fn empty_files(&self) -> Vec<NonHashedFile> {
		match self {
			Self::NonHashed(lst) => lst
				.files
				.iter()
				.filter_map(|(k, v)| {
					if lst.empty_files.contains(k) {
						return Some(v.clone());
					}
					None
				})
				.collect(),
			Self::Hashed(_) | Self::None => Vec::new(),
		}
	}

	pub fn duplicated_files(&self) -> Vec<Vec<HashedFile>> {
		match self {
			Self::Hashed(lst) => lst
				.duplicated_files
				.clone()
				.into_values()
				.map(|file_set| {
					file_set
						.iter()
						.map(|file_id| lst.files.get(file_id).unwrap().to_owned())
						.collect()
				})
				.collect(),
			Self::NonHashed(_) | Self::None => Vec::new(),
		}
	}
}

macro_rules! common_lst_impl {
	($lst_type: ty, $file_type: ty) => {
		impl $lst_type {
			pub fn get_id(&self) -> String {
				self.id.to_string()
			}

			pub fn get_base_dir(&self) -> &Path {
				self.base_dir.as_path()
			}

			pub fn get_files(&self) -> Vec<$file_type> {
				self.files.iter().map(|(_, v)| v.clone()).collect()
			}
		}
	};
}

#[derive(Debug, Clone)]
pub struct NonHashedFileList {
	id: Uuid,
	base_dir: PathBuf,
	files: HashMap<FileId, NonHashedFile>,
	empty_files: HashSet<FileId>,
	excluded_files: HashSet<NonHashedFile>,
}

common_lst_impl!(NonHashedFileList, NonHashedFile);

impl NonHashedFileList {
	pub fn len(&self) -> usize {
		self.files.len()
	}

	pub async fn from_dir<P: AsRef<Path>>(
		dir_path: P,
		include_hidden_files: bool,
		include_system_files: bool,
	) -> io::Result<Self> {
		let dir_path = dir_path.as_ref().to_path_buf();
		let mut empty_files = HashSet::new();
		let mut excluded_files = HashSet::new();
		let mut excluded_prefixes = HashSet::new();
		let files = walkdir::WalkDir::new(&dir_path)
			.follow_links(false)
			.into_iter()
			.filter_map(|entry| match entry {
				Ok(entry) => {
					let path = entry.clone().into_path();
					if path.is_file() {
						match NonHashedFile::new(&dir_path, &entry.clone().into_path()) {
							Ok(file) => {
								if !include_system_files && file.is_system {
									info!("System file excluded: {}", file.relative_path.display());
									excluded_files.insert(file);
									return None;
								}
								if !include_hidden_files && file.is_hidden {
									info!("Hidden file excluded: {}", file.relative_path.display());
									excluded_files.insert(file);
									return None;
								}
								for exl_p in &excluded_prefixes {
									if path.starts_with(exl_p) {
										info!(
											"File in hidden directory excluded: {}",
											file.relative_path.display()
										);
										excluded_files.insert(file);
										return None;
									}
								}
								let id = file.get_id();
								info!("File loaded: {}", file.relative_path.display());
								if file.is_empty() {
									empty_files.insert(id.clone());
								}
								return Some((id, file));
							}
							Err(e) => {
								error!("{}: unable to read file: {e}", entry.into_path().display());
								return None;
							}
						}
					}
					if path.is_dir() {
						if let Ok(true) = is_hidden_file(&path) {
							info!("Hidden directory excluded: {}", path.display());
							excluded_prefixes.insert(path.clone());
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
			id: Uuid::new_v4(),
			base_dir: dir_path,
			files,
			empty_files,
			excluded_files,
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
		let mut duplicated_files: HashMap<String, HashSet<FileId>> =
			HashMap::with_capacity(self.files.len());
		for r in handle_list.join_all().await {
			let r: io::Result<_> = r?;
			let (k, f) = r?;
			match duplicated_files.get_mut(&f.hash) {
				Some(set) => {
					set.insert(f.get_id());
				}
				None => {
					let mut set = HashSet::with_capacity(1);
					set.insert(f.get_id());
					duplicated_files.insert(f.hash.clone(), set);
				}
			}
			files.insert(k, f);
		}
		duplicated_files.retain(|_, v| v.len() > 1);
		Ok(HashedFileList {
			id: Uuid::new_v4(),
			base_dir: self.base_dir.clone(),
			files,
			empty_files: self.empty_files.clone(),
			duplicated_files,
		})
	}
}

#[derive(Debug, Clone)]
pub struct HashedFileList {
	id: Uuid,
	base_dir: PathBuf,
	files: HashMap<FileId, HashedFile>,
	empty_files: HashSet<FileId>,
	duplicated_files: HashMap<String, HashSet<FileId>>,
}

common_lst_impl!(HashedFileList, HashedFile);

impl HashedFileList {
	pub fn new() -> Self {
		Self {
			id: Uuid::new_v4(),
			base_dir: PathBuf::new(),
			files: HashMap::new(),
			empty_files: HashSet::new(),
			duplicated_files: HashMap::new(),
		}
	}

	pub fn insert_file(&mut self, file: HashedFile) {
		self.files.insert(file.get_id(), file);
	}

	pub fn is_empty(&self) -> bool {
		self.files.is_empty()
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileId(Vec<u8>);

macro_rules! common_file_impl {
	($file_type: ty) => {
		impl $file_type {
			pub fn get_id(&self) -> FileId {
				FileId(
					[
						self.base_dir.as_os_str().as_encoded_bytes(),
						self.relative_path.as_os_str().as_encoded_bytes(),
					]
					.join(&0),
				)
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

			pub fn is_empty(&self) -> bool {
				self.size == 0
			}
		}
	};
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NonHashedFile {
	base_dir: PathBuf,
	relative_path: PathBuf,
	size: u64,
	is_hidden: bool,
	is_system: bool,
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
			is_hidden: is_hidden_file(path)?,
			is_system: is_system_file(path)?,
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
	pub fn new<P, S>(relative_path: P, size: u64, hash: S, hash_func: HashFunc) -> Self
	where
		P: AsRef<Path>,
		S: AsRef<str>,
	{
		Self {
			base_dir: PathBuf::new(),
			relative_path: relative_path.as_ref().to_path_buf(),
			size,
			hash: hash.as_ref().into(),
			hash_func,
		}
	}

	pub fn get_hash(&self) -> &str {
		self.hash.as_str()
	}

	pub fn get_hash_func(&self) -> HashFunc {
		self.hash_func
	}
}

#[cfg(unix)]
#[inline]
fn is_hidden_file<P: AsRef<Path>>(path: P) -> io::Result<bool> {
	match path.as_ref().file_name() {
		Some(name) => Ok(name.to_string_lossy().starts_with('.')),
		None => Ok(false),
	}
}

#[cfg(unix)]
#[inline]
fn is_system_file(_path: &Path) -> io::Result<bool> {
	Ok(false)
}

#[cfg(windows)]
#[inline]
fn is_hidden_file<P: AsRef<Path>>(path: P) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_HIDDEN)
}

#[cfg(windows)]
#[inline]
fn is_system_file(path: &Path) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_SYSTEM)
}

#[cfg(windows)]
#[inline]
fn file_has_attr<P: AsRef<Path>>(path: P, attr: u32) -> io::Result<bool> {
	let metadata = std::fs::metadata(path.as_ref())?;
	let attributes = metadata.file_attributes();
	Ok((attributes & attr) > 0)
}
