use crate::config::Config;
use crate::files::HashedFileList;
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum ClipboardPersistence {
	Default,
	Activated,
	Deactivated,
}

impl ClipboardPersistence {
	fn is_persistent(&self) -> bool {
		match self {
			Self::Activated => true,
			Self::Deactivated => false,
			Self::Default => cfg!(unix),
		}
	}
}

impl std::str::FromStr for ClipboardPersistence {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"activated" => Ok(Self::Activated),
			"deactivated" => Ok(Self::Deactivated),
			_ => Ok(Self::Default),
		}
	}
}

impl fmt::Display for ClipboardPersistence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Activated => "activated",
			Self::Deactivated => "deactivated",
			Self::Default => "default",
		};
		write!(f, "{s}")
	}
}

impl From<Option<bool>> for ClipboardPersistence {
	fn from(opt: Option<bool>) -> Self {
		match opt {
			None => Self::Default,
			Some(true) => Self::Activated,
			Some(false) => Self::Deactivated,
		}
	}
}

impl From<ClipboardPersistence> for Option<bool> {
	fn from(val: ClipboardPersistence) -> Self {
		match val {
			ClipboardPersistence::Default => None,
			ClipboardPersistence::Activated => Some(true),
			ClipboardPersistence::Deactivated => Some(false),
		}
	}
}

pub struct Clipboard {
	internal: Option<arboard::Clipboard>,
}

impl Clipboard {
	pub fn new() -> Self {
		Self { internal: None }
	}

	pub fn set_clipboard(&mut self, config: &Config, file_list: &HashedFileList, threshold: usize) {
		if file_list.len() < threshold {
			self.set_clipboard_list(config, file_list);
		} else if let Ok(path) = file_list.get_content_file_absolute_path(config) {
			self.set_clipboard_ctn_file(config, &path);
		}
	}

	pub fn set_clipboard_list(&mut self, config: &Config, file_list: &HashedFileList) {
		let content_txt = "Debug test list";
		let content_html = "<h1>Debug</h1><p>test list</p>";
		self.set_content(config, content_txt, content_html);
	}

	pub fn set_clipboard_ctn_file(&mut self, config: &Config, content_file_path: &Path) {
		let content_txt = "Debug test content file";
		let content_html = "<h1>Debug</h1><p>test content file</p>";
		self.set_content(config, content_txt, content_html);
	}

	fn set_content(&mut self, config: &Config, txt: &str, html: &str) {
		if let Ok(mut clipboard) = arboard::Clipboard::new() {
			let _ = clipboard.set_html(html, Some(txt));
			if config.get_clipboard_persistence().is_persistent() {
				self.internal = Some(clipboard);
			} else {
				self.internal = None;
			}
		}
	}
}
