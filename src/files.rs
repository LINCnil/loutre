use crate::check::{CheckResult, CheckResultError};
use crate::config::Config;
use crate::content_file_format::ContentFileFormat;
use crate::events::ExternalEventSender;
use crate::hash::HashFunc;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io;
#[cfg(windows)]
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
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

	pub fn nb_empty_files(&self) -> usize {
		match self {
			Self::NonHashed(lst) => lst.empty_files.len(),
			Self::Hashed(_) | Self::None => 0,
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

	pub fn nb_excluded_files(&self) -> usize {
		match self {
			Self::NonHashed(lst) => lst.excluded_files.len(),
			Self::Hashed(_) | Self::None => 0,
		}
	}

	pub fn excluded_files(&self) -> Vec<NonHashedFile> {
		match self {
			Self::NonHashed(lst) => lst.excluded_files.iter().cloned().collect(),
			Self::Hashed(_) | Self::None => Vec::new(),
		}
	}

	pub fn has_duplicated_files(&self) -> bool {
		match self {
			Self::Hashed(lst) => !lst.duplicated_files.is_empty(),
			Self::NonHashed(_) | Self::None => false,
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
			pub fn len(&self, config_opt: Option<&Config>) -> usize {
				if let Some(config) = config_opt {
					if let Ok(ctn_file) = self.get_content_file_absolute_path(config) {
						return self.files.values().filter(|e| {
							match e.get_absolute_path() {
								Ok(path) => path != ctn_file,
								Err(_) => false,
							}
						}).count();
					}
				}
				self.files.len()
			}

			pub fn get_id(&self) -> String {
				self.id.to_string()
			}

			pub fn get_base_dir(&self) -> &Path {
				self.base_dir.as_path()
			}

			pub fn get_content_file_absolute_path(&self, config: &Config) -> io::Result<PathBuf> {
				let mut path = self.base_dir.clone().canonicalize()?;
				path.push(config.get_content_file_name());
				Ok(path)
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
	pub fn total_size(&self) -> u64 {
		self.files.values().fold(0, |acc, f| acc + f.size)
	}

	pub fn content_file_exists(&self, config: &Config) -> bool {
		if let Ok(ctn_file_path) = self.get_content_file_absolute_path(config) {
			return ctn_file_path.is_file();
		}
		false
	}

	pub async fn from_dir<P: AsRef<Path>>(
		dir_path: P,
		include_hidden_files: bool,
		include_system_files: bool,
	) -> io::Result<Self> {
		let dir_path = dir_path.as_ref().to_path_buf();
		let mut empty_files = HashSet::new();
		let mut excluded_files = HashSet::new();
		let mut system_prefixes = HashSet::new();
		let mut hidden_prefixes = HashSet::new();
		let files = walkdir::WalkDir::new(&dir_path)
			.follow_links(false)
			.into_iter()
			.filter_map(|entry| match entry {
				Ok(entry) => {
					let path = entry.clone().into_path();
					if path.is_file() {
						match NonHashedFile::new(&dir_path, &entry.clone().into_path()) {
							Ok(mut file) => {
								if !include_system_files && file.is_system {
									tracing::info!(
										"System file excluded: {}",
										file.relative_path.display()
									);
									excluded_files.insert(file);
									return None;
								}
								if !include_hidden_files && file.is_hidden {
									tracing::info!(
										"Hidden file excluded: {}",
										file.relative_path.display()
									);
									excluded_files.insert(file);
									return None;
								}
								for exl_p in &system_prefixes {
									if path.starts_with(exl_p) {
										tracing::info!(
											"File in system directory excluded: {}",
											file.relative_path.display()
										);
										file.is_system = true;
										excluded_files.insert(file);
										return None;
									}
								}
								for exl_p in &hidden_prefixes {
									if path.starts_with(exl_p) {
										tracing::info!(
											"File in hidden directory excluded: {}",
											file.relative_path.display()
										);
										file.is_hidden = true;
										excluded_files.insert(file);
										return None;
									}
								}
								let id = file.get_id();
								tracing::info!("File loaded: {}", file.relative_path.display());
								if file.is_empty() {
									empty_files.insert(id.clone());
								}
								return Some((id, file));
							}
							Err(e) => {
								tracing::error!(
									"{}: unable to read file: {e}",
									entry.into_path().display()
								);
								return None;
							}
						}
					}
					if path.is_dir() {
						if let Ok(true) = is_system_file(&path) {
							tracing::info!("System directory excluded: {}", path.display());
							system_prefixes.insert(path.clone());
						} else if let Ok(true) = is_hidden_file(&path) {
							tracing::info!("Hidden directory excluded: {}", path.display());
							hidden_prefixes.insert(path.clone());
						}
					}
					None
				}
				Err(e) => {
					tracing::error!("Error while loading file: {e}");
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

	pub fn hash(
		&self,
		config: &Config,
		hash_func: HashFunc,
		tx: ExternalEventSender,
	) -> io::Result<HashedFileList> {
		let ctn_file_absolute_path = self.get_content_file_absolute_path(config)?;
		let files: HashMap<FileId, HashedFile> = HashMap::with_capacity(self.files.len());
		let files_mx = std::sync::Mutex::new(files);
		let set_ro = config.set_files_as_readonly();
		self.files
			.par_iter()
			.try_for_each(|(k, f)| -> io::Result<()> {
				let abs_path = f.get_absolute_path()?;
				if abs_path != ctn_file_absolute_path {
					if set_ro {
						set_readonly(abs_path)?;
					}
					let file = f.hash(hash_func, tx.clone())?;
					let mut files_lock = files_mx.lock().unwrap();
					files_lock.insert(k.clone(), file);
				}
				Ok(())
			})?;
		let files = files_mx.into_inner().unwrap();

		let mut duplicated_files: HashMap<String, HashSet<FileId>> =
			HashMap::with_capacity(self.files.len());
		for (k, f) in files.iter() {
			match duplicated_files.get_mut(&f.hash) {
				Some(set) => {
					set.insert(k.clone());
				}
				None => {
					let mut set = HashSet::with_capacity(1);
					set.insert(k.clone());
					duplicated_files.insert(f.hash.clone(), set);
				}
			};
		}
		duplicated_files.retain(|_, v| v.len() > 1);
		let hashed_lst = HashedFileList {
			id: Uuid::new_v4(),
			base_dir: self.base_dir.clone(),
			files,
			duplicated_files,
			result: CheckResult::None,
		};
		hashed_lst
			.write_content_file_opt(ctn_file_absolute_path.as_path(), config.content_file_format)?;
		if set_ro {
			set_readonly(ctn_file_absolute_path)?;
		}
		Ok(hashed_lst)
	}
}

#[derive(Debug, Clone)]
pub struct HashedFileList {
	id: Uuid,
	base_dir: PathBuf,
	files: HashMap<FileId, HashedFile>,
	duplicated_files: HashMap<String, HashSet<FileId>>,
	result: CheckResult,
}

common_lst_impl!(HashedFileList, HashedFile);

impl HashedFileList {
	pub fn new() -> Self {
		Self {
			id: Uuid::new_v4(),
			base_dir: PathBuf::new(),
			files: HashMap::new(),
			duplicated_files: HashMap::new(),
			result: CheckResult::None,
		}
	}

	pub fn get_files(&self) -> std::collections::hash_map::Values<FileId, HashedFile> {
		self.files.values()
	}

	pub fn insert_file(&mut self, file: HashedFile) {
		self.files.insert(file.get_id(), file);
	}

	pub fn set_result_ok(&mut self) {
		self.result = CheckResult::Ok;
	}

	pub fn push_result_error(&mut self, error: CheckResultError) {
		match &self.result {
			CheckResult::Error(v) => {
				let mut v = v.clone();
				v.push(error);
				self.result = CheckResult::Error(v);
			}
			_ => {
				self.result = CheckResult::Error(vec![error]);
			}
		}
	}

	pub fn is_empty(&self) -> bool {
		self.files.is_empty()
	}

	pub fn get_result(&self) -> CheckResult {
		self.result.clone()
	}

	pub fn get_main_hashing_function(&self) -> HashFunc {
		let mut occurrences = HashMap::with_capacity(self.files.len());
		for file in self.files.values() {
			match occurrences.get_mut(&file.hash_func) {
				Some(nb) => {
					*nb += 1;
				}
				None => {
					occurrences.insert(file.hash_func, 1);
				}
			}
		}
		let mut occurrences: Vec<(HashFunc, usize)> =
			occurrences.iter().map(|(k, v)| (*k, *v)).collect();
		occurrences.sort_by(|a, b| a.1.cmp(&b.1));
		let (hash_func, _) = occurrences.pop().unwrap();
		hash_func
	}

	fn write_content_file_opt(
		&self,
		ctn_file_path: &Path,
		format: ContentFileFormat,
	) -> io::Result<()> {
		if !ctn_file_path.exists() {
			let mut f = File::create_new(ctn_file_path)?;
			return format.write_content_file(&mut f, self);
		}
		Ok(())
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

			pub fn get_absolute_path(&self) -> io::Result<PathBuf> {
				let mut path = self.base_dir.clone();
				path.push(self.relative_path.clone());
				path.canonicalize()
			}

			pub fn get_relative_path(&self) -> &Path {
				self.relative_path.as_path()
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
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	pub fn is_hidden(&self) -> bool {
		self.is_hidden
	}

	pub fn is_system(&self) -> bool {
		self.is_system
	}

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

	pub fn hash(&self, hash_func: HashFunc, tx: ExternalEventSender) -> io::Result<HashedFile> {
		let path = self.get_absolute_path()?;
		let hash = hash_func.hash_file(path, Some(tx))?;
		Ok(HashedFile {
			base_dir: self.base_dir.clone(),
			relative_path: self.relative_path.clone(),
			size: self.size,
			hash,
			hash_func,
		})
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
		HashedFile::new_base_dir(PathBuf::new(), relative_path, size, hash, hash_func)
	}

	pub fn new_base_dir<P1, P2, S>(
		base_dir: P1,
		relative_path: P2,
		size: u64,
		hash: S,
		hash_func: HashFunc,
	) -> Self
	where
		P1: AsRef<Path>,
		P2: AsRef<Path>,
		S: AsRef<str>,
	{
		Self {
			base_dir: base_dir.as_ref().into(),
			relative_path: relative_path.as_ref().to_path_buf(),
			size,
			hash: hash.as_ref().into(),
			hash_func,
		}
	}

	pub fn get_base_dir(&self) -> PathBuf {
		self.base_dir.clone()
	}

	pub fn get_size(&self) -> u64 {
		self.size
	}

	pub fn get_hash(&self) -> &str {
		self.hash.as_str()
	}

	pub fn get_hash_func(&self) -> HashFunc {
		self.hash_func
	}
}

#[inline]
fn set_readonly(path: PathBuf) -> io::Result<()> {
	let metadata = path.metadata()?;
	let mut permissions = metadata.permissions();
	permissions.set_readonly(true);
	fs::set_permissions(path, permissions)?;
	Ok(())
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
