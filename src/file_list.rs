use crate::clipboard::Clipboard;
use crate::content_file::ContentFile;
use crate::file::File;
use crate::hasher::{hash_single_file, HashFunc};
use crate::i18n::{Attr, I18n};
use std::collections::HashMap;
use std::io;
#[cfg(windows)]
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, channel};
use std::{fmt, thread};

// Microsoft Windows File Attribute Constants
// https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
#[cfg(windows)]
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
#[cfg(windows)]
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;

#[derive(Clone, Eq, PartialEq)]
pub struct FileAsk {
	pub path: PathBuf,
	pub is_hidden: bool,
	pub is_system: bool,
}

pub enum FileAskAnswer {
	Allow,
	AllowAll,
	Deny,
	DenyAll,
}

#[derive(PartialEq)]
enum FileListBuilderState {
	Ask(FileAsk),
	Scanning,
	Ok,
	Err(String),
}

enum FileListBuilderResponse {
	Ask(FileAsk),
	NewFiles(Vec<File>),
	Done,
	Error(io::Error),
}

pub struct FileListBuilder {
	path: PathBuf,
	content_file_path: PathBuf,
	files: Vec<File>,
	state: FileListBuilderState,
	rx: mpsc::Receiver<FileListBuilderResponse>,
	a_tx: mpsc::Sender<bool>,
	allow_hidden_files: Option<bool>,
	allow_system_files: Option<bool>,
}

impl FileListBuilder {
	pub fn from_dir(path: &Path, content_file_name: &str) -> io::Result<Self> {
		let (tx, rx) = channel();
		let (a_tx, fa_rx) = channel();
		let content_file_path = FileList::get_content_file(path, content_file_name);
		let lst = FileListBuilder {
			path: path.to_owned(),
			content_file_path: content_file_path.to_owned(),
			files: Vec::new(),
			state: FileListBuilderState::Scanning,
			rx,
			a_tx,
			allow_hidden_files: None,
			allow_system_files: None,
		};
		let path = path.to_owned();
		thread::spawn(move || {
			match FileListBuilder::visit_dir(tx.clone(), fa_rx, &path, &content_file_path) {
				Ok(_) => {
					let _ = tx.send(FileListBuilderResponse::Done);
				}
				Err(e) => {
					let _ = tx.send(FileListBuilderResponse::Error(e));
				}
			};
		});
		Ok(lst)
	}

	fn visit_dir(
		tx: mpsc::Sender<FileListBuilderResponse>,
		fa_rx: mpsc::Receiver<bool>,
		base_path: &Path,
		ctn: &Path,
	) -> io::Result<()> {
		let mut ignored_prefixes = vec![];
		let files = walkdir::WalkDir::new(base_path)
			.follow_links(false)
			.into_iter()
			.filter_map(|entry| {
				let entry = entry.ok()?;
				let file_type = entry.file_type();
				if file_type.is_file() || file_type.is_dir() {
					let path = entry.path();
					for ignored_prefix in &ignored_prefixes {
						if path.starts_with(ignored_prefix) {
							return None;
						}
					}
					if path == ctn {
						return None;
					}
					let is_hidden = is_hidden(path).ok()?;
					let is_system = is_system(path).ok()?;
					if is_hidden || is_system {
						let _ = tx.send(FileListBuilderResponse::Ask(FileAsk {
							path: path.to_owned(),
							is_hidden,
							is_system,
						}));
						if !fa_rx.recv().unwrap() {
							if file_type.is_dir() {
								ignored_prefixes.push(path.to_owned());
							}
							return None;
						}
					}
					if file_type.is_file() {
						File::new(entry.path(), base_path).ok()
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect::<Vec<File>>();
		if !files.is_empty() {
			let _ = tx.send(FileListBuilderResponse::NewFiles(files));
		}
		Ok(())
	}

	pub fn update_state(&mut self, i18n: &I18n) {
		if let FileListBuilderState::Ask(_) = &self.state {
			return;
		}
		match self.rx.try_recv() {
			Ok(msg) => match msg {
				FileListBuilderResponse::Ask(af) => {
					if af.is_hidden {
						match self.allow_hidden_files {
							Some(allow) => {
								let _ = self.a_tx.send(allow);
							}
							None => {
								self.state = FileListBuilderState::Ask(af);
							}
						}
					} else if af.is_system {
						match self.allow_system_files {
							Some(allow) => {
								let _ = self.a_tx.send(allow);
							}
							None => {
								self.state = FileListBuilderState::Ask(af);
							}
						}
					}
				}
				FileListBuilderResponse::NewFiles(mut f) => {
					self.files.append(&mut f);
				}
				FileListBuilderResponse::Done => {
					self.state = FileListBuilderState::Ok;
				}
				FileListBuilderResponse::Error(e) => {
					let msg = i18n.fmt(
						"error_desc",
						&[
							("error", Attr::String(i18n.msg("msg_err_fl"))),
							("description", Attr::String(e.to_string())),
						],
					);
					self.state = FileListBuilderState::Err(msg);
				}
			},
			Err(e) => {
				if e == mpsc::TryRecvError::Disconnected
					&& self.state == FileListBuilderState::Scanning
				{
					self.state = FileListBuilderState::Err(i18n.msg("msg_err_fl_interrupted"));
				}
			}
		}
	}

	pub fn is_ready(&self) -> bool {
		match &self.state {
			FileListBuilderState::Ask(_) => false,
			FileListBuilderState::Scanning => false,
			FileListBuilderState::Ok => true,
			FileListBuilderState::Err(_) => true,
		}
	}

	pub fn ask_for(&self) -> Option<FileAsk> {
		if let FileListBuilderState::Ask(f) = &self.state {
			return Some((*f).to_owned());
		}
		None
	}

	pub fn answer(&mut self, answer: FileAskAnswer) {
		if let FileListBuilderState::Ask(f) = &self.state {
			let a = match answer {
				FileAskAnswer::Allow => true,
				FileAskAnswer::AllowAll => {
					if f.is_hidden {
						self.allow_hidden_files = Some(true);
					} else if f.is_system {
						self.allow_system_files = Some(true);
					}
					true
				}
				FileAskAnswer::Deny => false,
				FileAskAnswer::DenyAll => {
					if f.is_hidden {
						self.allow_hidden_files = Some(false);
					} else if f.is_system {
						self.allow_system_files = Some(false);
					}
					false
				}
			};
			self.state = FileListBuilderState::Scanning;
			let _ = self.a_tx.send(a);
		}
	}

	pub fn get_file_list(&self, i18n: &I18n) -> Result<FileList, String> {
		match &self.state {
			FileListBuilderState::Ask(_) => Err(i18n.msg("msg_err_fl_not_ready")),
			FileListBuilderState::Scanning => Err(i18n.msg("msg_err_fl_not_ready")),
			FileListBuilderState::Err(e) => Err(e.to_owned()),
			FileListBuilderState::Ok => Ok(FileList {
				path: self.path.to_owned(),
				content_file_path: self.content_file_path.to_owned(),
				files: self
					.files
					.iter()
					.map(|f| (f.get_id(), f.to_owned()))
					.collect(),
			}),
		}
	}
}

pub struct FileList {
	path: PathBuf,
	content_file_path: PathBuf,
	files: HashMap<Vec<u8>, File>,
}

impl FileList {
	pub fn get_content_file_path(&self) -> &Path {
		self.content_file_path.as_path()
	}

	pub fn get_content_file(path: &Path, content_file_name: &str) -> PathBuf {
		let mut content_file_path = path.to_path_buf();
		content_file_path.push(content_file_name);
		content_file_path
	}

	#[inline]
	pub fn iter_files(&self) -> std::collections::hash_map::Values<Vec<u8>, File> {
		self.files.values()
	}

	pub fn has_content_file(&self) -> bool {
		self.content_file_path.is_file()
	}

	pub fn has_hashes(&self) -> bool {
		self.files.values().any(|f| f.get_hash().is_some())
	}

	pub fn get_session_hash_func(&self, i18n: &I18n, default_hash: HashFunc) -> HashFunc {
		if self.has_content_file() {
			if let Ok(ctn_file) = ContentFile::load(i18n, self) {
				return ctn_file.get_hash_func();
			}
		}
		default_hash
	}

	pub fn get_nb_files(&self) -> usize {
		self.files.len()
	}

	pub fn get_total_size(&self) -> u64 {
		self.files.values().fold(0, |acc, f| acc + f.get_size())
	}

	pub fn set_readonly(&self) -> io::Result<()> {
		for f in self.iter_files() {
			f.set_readonly()?;
		}
		Ok(())
	}

	pub fn replace_file(&mut self, new_file: File) {
		self.files.insert(new_file.get_id(), new_file);
	}

	pub fn write_content_file(&mut self, i18n: &I18n, hash: HashFunc) -> io::Result<()> {
		let mut file_list: Vec<File> = self.iter_files().map(|f| f.to_owned()).collect();
		ContentFile::write(i18n, &self.content_file_path, &mut file_list, hash)
	}

	pub fn set_clipboard(&mut self, i18n: &I18n, clipboard: &mut Clipboard, nb_start: u32) {
		clipboard.set_clipboard(i18n, self, nb_start);
	}

	pub fn set_clipboard_ctn_file(
		&self,
		i18n: &I18n,
		clipboard: &mut Clipboard,
		nb_start: u32,
		hash: HashFunc,
	) {
		if let Ok(meta) = self.content_file_path.metadata() {
			let file = File::create_dummy(&self.content_file_path, &self.path, meta.len(), "");
			if let Ok(f) = hash_single_file(&file, hash) {
				clipboard.set_clipboard_ctn_file(i18n, &f, self.get_nb_files(), nb_start);
			}
		}
	}
}

impl fmt::Display for FileList {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.path.display())
	}
}

#[cfg(unix)]
#[inline]
fn is_hidden(path: &Path) -> io::Result<bool> {
	match path.file_name() {
		Some(name) => Ok(name.to_string_lossy().starts_with('.')),
		None => Ok(false),
	}
}

#[cfg(unix)]
#[inline]
fn is_system(_path: &Path) -> io::Result<bool> {
	Ok(false)
}

#[cfg(windows)]
#[inline]
fn is_hidden(path: &Path) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_HIDDEN)
}

#[cfg(windows)]
#[inline]
fn is_system(path: &Path) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_SYSTEM)
}

#[cfg(windows)]
#[inline]
fn file_has_attr(path: &Path, attr: u32) -> io::Result<bool> {
	let metadata = std::fs::metadata(path)?;
	let attributes = metadata.file_attributes();
	Ok((attributes & attr) > 0)
}
