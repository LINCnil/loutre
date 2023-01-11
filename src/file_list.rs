use crate::clipboard::Clipboard;
use crate::file::File;
use crate::hasher::{hash_single_file, HashFunc};
use crate::i18n::{Attr, I18n};
use std::io::{self, Write};
#[cfg(windows)]
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};
use std::slice::Iter;
use std::sync::mpsc::{self, channel};
use std::sync::Arc;
use std::{fmt, fs, thread};

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
			match FileListBuilder::visit_dir(
				tx.clone(),
				Arc::new(fa_rx),
				&path,
				&content_file_path,
				&path,
			) {
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
		fa_rx: Arc<mpsc::Receiver<bool>>,
		base_path: &Path,
		ctn: &Path,
		dir: &Path,
	) -> io::Result<()> {
		let mut files = vec![];
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			let is_hidden = is_hidden(&path)?;
			let is_system = is_system(&path)?;
			if is_hidden || is_system {
				let _ = tx.send(FileListBuilderResponse::Ask(FileAsk {
					path: path.to_owned(),
					is_hidden,
					is_system,
				}));
				if !fa_rx.recv().unwrap() {
					continue;
				}
			}
			if path.is_dir() {
				FileListBuilder::visit_dir(tx.clone(), fa_rx.clone(), base_path, ctn, &path)?;
			} else if path != ctn {
				let file = File::new(&path, base_path)?;
				files.push(file);
			}
		}
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
				files: self.files.to_owned(),
			}),
		}
	}
}

pub struct FileList {
	path: PathBuf,
	content_file_path: PathBuf,
	files: Vec<File>,
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

	pub fn iter_files(&self) -> Iter<File> {
		self.files.iter()
	}

	pub fn has_content_file(&self) -> bool {
		self.content_file_path.is_file()
	}

	pub fn has_hashes(&self) -> bool {
		for f in &self.files {
			if f.get_hash().is_some() {
				return true;
			}
		}
		false
	}

	pub fn get_session_hash_func(&self, default_hash: HashFunc) -> HashFunc {
		if self.has_content_file() {
			match crate::checker::get_content_file_hash(self) {
				Ok(h) => h,
				Err(_) => default_hash,
			}
		} else {
			default_hash
		}
	}

	pub fn get_nb_files(&self) -> usize {
		self.files.len()
	}

	pub fn get_total_size(&self) -> u64 {
		let mut s = 0;
		for f in &self.files {
			s += f.get_size();
		}
		s
	}

	pub fn set_readonly(&self) -> io::Result<()> {
		for f in &self.files {
			f.set_readonly()?;
		}
		Ok(())
	}

	pub fn replace_file(&mut self, new_file: File) {
		self.files.retain(|e| *e.get_path() != *new_file.get_path());
		self.files.push(new_file);
	}

	pub fn write_content_file(&mut self, i18n: &I18n, hash: HashFunc) -> io::Result<()> {
		self.files.sort_by(File::cmp_name);
		let mut content_file = fs::File::create(&self.content_file_path)?;
		let header = format!(
			"{}\t{}\t{}\r\n",
			i18n.msg("content_file_header.name"),
			i18n.msg("content_file_header.size"),
			hash.to_string(),
		);
		content_file.write_all(header.as_bytes())?;
		for f in &self.files {
			content_file.write_all(&f.get_content_file_line())?;
		}
		Ok(())
	}

	pub fn set_clipboard(&mut self, i18n: &I18n, clipboard: &mut Clipboard, nb_start: u32) {
		self.files.sort_by(File::cmp_name);
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
fn is_hidden(path: &Path) -> io::Result<bool> {
	match path.file_name() {
		Some(name) => Ok(name.to_string_lossy().starts_with('.')),
		None => Ok(false),
	}
}

#[cfg(unix)]
fn is_system(_path: &Path) -> io::Result<bool> {
	Ok(false)
}

#[cfg(windows)]
fn is_hidden(path: &Path) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_HIDDEN)
}

#[cfg(windows)]
fn is_system(path: &Path) -> io::Result<bool> {
	file_has_attr(path, FILE_ATTRIBUTE_SYSTEM)
}

#[cfg(windows)]
fn file_has_attr(path: &Path, attr: u32) -> io::Result<bool> {
	let metadata = fs::metadata(path)?;
	let attributes = metadata.file_attributes();
	Ok((attributes & attr) > 0)
}
