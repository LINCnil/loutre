use crate::config::Config;
use crate::files::HashedFileList;
use crate::templates::{filter_add_dir_level, filter_nb_letters, EntryTemplate};
use dioxus_i18n::t;
use dioxus_logger::tracing::error;
use minijinja::{context, Environment};
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct ClipboardStart(usize);

impl Default for ClipboardStart {
	fn default() -> Self {
		Self(1)
	}
}

impl fmt::Display for ClipboardStart {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<usize> for ClipboardStart {
	fn from(nb: usize) -> Self {
		Self(nb)
	}
}

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

#[derive(Clone, Debug)]
pub enum ClipboardError {
	ContentFileHash(String),
	ContentFileName,
	ContentFilePath,
	ContentFileSize,
	ContentFileTemplateText(String),
	ContentFileTemplateHtml(String),
	ListTemplateText(String),
	ListTemplateHtml(String),
}

impl fmt::Display for ClipboardError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::ContentFileHash(e) => format!("content file hash error: {e}"),
			Self::ContentFileName => "content file hash error".to_string(),
			Self::ContentFilePath => "content file path error".to_string(),
			Self::ContentFileSize => "content file size error".to_string(),
			Self::ContentFileTemplateText(e) => format!("content file text template error: {e}"),
			Self::ContentFileTemplateHtml(e) => format!("content file HTML template error: {e}"),
			Self::ListTemplateText(e) => format!("list text template error: {e}"),
			Self::ListTemplateHtml(e) => format!("list HTML template error: {e}"),
		};
		write!(f, "{s}")
	}
}

pub struct Clipboard {
	internal: Option<arboard::Clipboard>,
}

impl Clipboard {
	pub fn new() -> Self {
		Self { internal: None }
	}

	pub fn set_clipboard(
		&mut self,
		config: &Config,
		file_list: &HashedFileList,
		start: ClipboardStart,
		threshold: usize,
	) -> Result<(), ClipboardError> {
		if file_list.len() < threshold {
			self.set_clipboard_list(config, file_list, start)
		} else {
			self.set_clipboard_ctn_file(config, file_list, start)
		}
	}

	pub fn set_clipboard_list(
		&mut self,
		config: &Config,
		file_list: &HashedFileList,
		start: ClipboardStart,
	) -> Result<(), ClipboardError> {
		let ret = self.inner_set_clipboard_list(config, file_list, start);
		if let Err(ref e) = ret {
			error!("{e}");
		}
		ret
	}

	pub fn set_clipboard_ctn_file(
		&mut self,
		config: &Config,
		file_list: &HashedFileList,
		start: ClipboardStart,
	) -> Result<(), ClipboardError> {
		let ret = self.inner_set_clipboard_ctn_file(config, file_list, start);
		if let Err(ref e) = ret {
			error!("{e}");
		}
		ret
	}

	fn inner_set_clipboard_list(
		&mut self,
		config: &Config,
		file_list: &HashedFileList,
		start: ClipboardStart,
	) -> Result<(), ClipboardError> {
		let mut env = Environment::new();
		env.add_filter("add_dir_level", filter_add_dir_level);
		env.add_filter("nb_letters", filter_nb_letters);
		let mut evidences: Vec<EntryTemplate> =
			file_list.get_files().map(|f| f.clone().into()).collect();
		evidences.sort_by(|a, b| a.name.cmp(&b.name));
		let ctx = context!(
			hash_func => file_list.get_main_hashing_function().to_string(),
			nb_start => start.0,
			evidences,
		);
		let model_txt = t!("cpn_clipboard_list_full_txt");
		let model_html = t!("cpn_clipboard_list_full_html");
		env.add_template("txt", &model_txt)
			.map_err(|e| ClipboardError::ListTemplateText(e.to_string()))?;
		env.add_template("html", &model_html)
			.map_err(|e| ClipboardError::ListTemplateHtml(e.to_string()))?;
		let tmpl_txt = env
			.get_template("txt")
			.map_err(|e| ClipboardError::ListTemplateText(e.to_string()))?;
		let html_html = env
			.get_template("html")
			.map_err(|e| ClipboardError::ListTemplateHtml(e.to_string()))?;
		let content_txt = tmpl_txt
			.render(&ctx)
			.map_err(|e| ClipboardError::ListTemplateText(e.to_string()))?;
		let content_html = html_html
			.render(&ctx)
			.map_err(|e| ClipboardError::ListTemplateHtml(e.to_string()))?;
		self.set_content(config, &content_txt, &content_html);
		Ok(())
	}

	fn inner_set_clipboard_ctn_file(
		&mut self,
		config: &Config,
		file_list: &HashedFileList,
		start: ClipboardStart,
	) -> Result<(), ClipboardError> {
		let mut env = Environment::new();
		env.add_filter("nb_letters", filter_nb_letters);
		let content_file_path = file_list
			.get_content_file_absolute_path(config)
			.map_err(|_| ClipboardError::ContentFilePath)?;
		let name = Path::new(
			content_file_path
				.file_name()
				.ok_or(ClipboardError::ContentFileName)?,
		)
		.display()
		.to_string();
		let size = content_file_path
			.metadata()
			.map_err(|_| ClipboardError::ContentFileSize)?
			.len();
		let hash_func = file_list.get_main_hashing_function();
		let hash = hash_func
			.hash_file(content_file_path, None)
			.map_err(|e| ClipboardError::ContentFileHash(e.to_string()))?;
		let nb_evidences = file_list.len();
		let hash_func = hash_func.to_string();
		let ctx = context!(
			hash_func,
			nb_evidences => nb_evidences,
			nb_start => start.0,
			evidence => context!(
				name,
				size,
				hash,
				hash_func,
			),
		);
		let model_txt = t!("cpn_clipboard_ctn_file_full_txt", nb_evidences: nb_evidences);
		let model_html = t!("cpn_clipboard_ctn_file_full_html", nb_evidences: nb_evidences);
		env.add_template("txt", &model_txt)
			.map_err(|e| ClipboardError::ContentFileTemplateText(e.to_string()))?;
		env.add_template("html", &model_html)
			.map_err(|e| ClipboardError::ContentFileTemplateHtml(e.to_string()))?;
		let tmpl_txt = env
			.get_template("txt")
			.map_err(|e| ClipboardError::ContentFileTemplateText(e.to_string()))?;
		let html_html = env
			.get_template("html")
			.map_err(|e| ClipboardError::ContentFileTemplateHtml(e.to_string()))?;
		let content_txt = tmpl_txt
			.render(&ctx)
			.map_err(|e| ClipboardError::ContentFileTemplateText(e.to_string()))?;
		let content_html = html_html
			.render(&ctx)
			.map_err(|e| ClipboardError::ListTemplateHtml(e.to_string()))?;
		self.set_content(config, &content_txt, &content_html);
		Ok(())
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
