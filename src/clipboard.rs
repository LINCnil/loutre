use crate::file::File;
use crate::file_list::FileList;
use crate::i18n::{Attr, I18n};
use crate::nb_repr::NbRepr;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const HTML_BOLD_OPEN: &str = "<b>";
const HTML_BOLD_CLOSE: &str = "</b>";
const HTML_FONT_SMALL_OPEN: &str = "<small>";
const HTML_FONT_SMALL_CLOSE: &str = "</small>";
const HTML_ITALIC_OPEN: &str = "<i>";
const HTML_ITALIC_CLOSE: &str = "</i>";
const HTML_LIST_OPEN: &str = "<ul>";
const HTML_LIST_CLOSE: &str = "</ul>";
const HTML_LIST_ELEM_OPEN: &str = "<li>";
const HTML_LIST_ELEM_CLOSE: &str = "</li>";
const HTML_NEW_LINE: &str = "<br>";
const HTML_P_OPEN: &str = "<p>";
const HTML_P_CLOSE: &str = "</p>";
const HTML_SUP_OPEN: &str = "<sup>";
const HTML_SUP_CLOSE: &str = "</sup>";
const TPL_HASH_METHOD: &str = "SHA-256";

macro_rules! if_html {
	($tag: ident, $html: ident) => {
		if $html {
			$tag
		} else {
			""
		}
	};
}

pub struct Clipboard {
	internal: Option<arboard::Clipboard>,
	nb_repr: NbRepr,
	persistence: Option<bool>,
}

impl Clipboard {
	pub fn new(nb_repr: NbRepr, persistence: Option<bool>) -> Self {
		Self {
			internal: None,
			nb_repr,
			persistence,
		}
	}
}

impl Clipboard {
	pub fn set_clipboard(&mut self, i18n: &I18n, file_list: &FileList, nb_start: u32) {
		let mut file_list: Vec<File> = file_list.iter_files().map(|f| f.to_owned()).collect();
		file_list.sort_by(File::cmp_name);
		let html = get_clipboard_content(i18n, &file_list, true, nb_start, self.nb_repr);
		let alt_text = get_clipboard_content(i18n, &file_list, false, nb_start, self.nb_repr);
		self.set_html(&html, &alt_text);
	}

	pub fn set_clipboard_ctn_file(
		&mut self,
		i18n: &I18n,
		file: &File,
		nb_files: usize,
		nb_start: u32,
	) {
		let html =
			get_clipboard_content_ctn_file(i18n, file, true, nb_files, nb_start, self.nb_repr);
		let alt_text =
			get_clipboard_content_ctn_file(i18n, file, false, nb_files, nb_start, self.nb_repr);
		self.set_html(&html, &alt_text);
	}

	pub fn set_html(&mut self, html: &str, alt_text: &str) {
		match &mut self.internal {
			Some(clipboard) => {
				let _ = clipboard.clear();
				let _ = clipboard.set_html(html, Some(alt_text));
			}
			None => {
				let mut clipboard = arboard::Clipboard::new().unwrap();
				let _ = clipboard.set_html(html, Some(alt_text));
				if self.persistence.unwrap_or(cfg!(unix)) {
					self.internal = Some(clipboard);
				}
			}
		}
	}
}

enum Exhibit {
	Dir(Vec<File>),
	File(File),
}

fn get_dir_name(file: &File) -> Option<PathBuf> {
	let name = file.get_file_name();
	let first_cpn = PathBuf::from(name.components().next().unwrap().as_os_str());
	if first_cpn == name {
		None
	} else {
		Some(first_cpn)
	}
}

fn get_file_name_no_dir(file: &File, dir_name: &Path) -> String {
	file.get_file_name()
		.strip_prefix(dir_name)
		.unwrap_or(&file.get_file_name())
		.display()
		.to_string()
}

fn get_exhibits(file_list: &[File]) -> Vec<Exhibit> {
	let mut lst = Vec::new();
	for f in file_list {
		match get_dir_name(f) {
			Some(dir_name) => match lst
				.iter_mut()
				.filter_map(|e| match e {
					Exhibit::Dir(d) => Some(d),
					Exhibit::File(_) => None,
				})
				.find(|e| dir_name == get_dir_name(e.first().unwrap()).unwrap())
			{
				Some(e) => {
					e.push(f.to_owned());
				}
				None => {
					lst.push(Exhibit::Dir(vec![f.to_owned()]));
				}
			},
			None => {
				lst.push(Exhibit::File(f.to_owned()));
			}
		}
	}
	lst
}

fn format_file(i18n: &I18n, n: u32, file: &File, html: bool) -> String {
	let mut ctn = String::new();
	let sup_open = if html {
		HTML_SUP_OPEN.to_string()
	} else {
		String::new()
	};
	let sup_close = if html {
		HTML_SUP_CLOSE.to_string()
	} else {
		String::new()
	};
	if let Some(hash) = file.get_hash() {
		if n != 1 {
			let _ = writeln!(ctn, "\n");
		}
		let file_size = file.get_size();
		let _ = write!(
			ctn,
			"{}{}{}{} {}{}\n{}{}{}{} {}, {} {}{}{}{}{}",
			if_html!(HTML_P_OPEN, html),
			// Numéro de la pièce
			if_html!(HTML_BOLD_OPEN, html),
			i18n.fmt(
				"msg_exhibit",
				&[
					("sup_open", Attr::String(sup_open)),
					("sup_close", Attr::String(sup_close)),
					("nb", Attr::U32(n)),
				]
			),
			if_html!(HTML_BOLD_CLOSE, html),
			// Nature de la pièce
			i18n.fmt(
				"msg_file",
				&[("file_name", Attr::String(file.display_file_name()))]
			),
			if_html!(HTML_NEW_LINE, html),
			if_html!(HTML_FONT_SMALL_OPEN, html),
			// Taille de la pièce
			if_html!(HTML_ITALIC_OPEN, html),
			file_size,
			if_html!(HTML_ITALIC_CLOSE, html),
			i18n.fmt("msg_file_unit", &[("nb", Attr::U64(file_size))]),
			// Empreinte de la pièce
			TPL_HASH_METHOD,
			if_html!(HTML_ITALIC_OPEN, html),
			hash,
			if_html!(HTML_ITALIC_CLOSE, html),
			if_html!(HTML_FONT_SMALL_CLOSE, html),
			if_html!(HTML_P_CLOSE, html),
		);
	}
	ctn
}

fn format_dir(i18n: &I18n, n: u32, files: &[File], html: bool, nb_repr: NbRepr) -> String {
	let dir_name = get_dir_name(files.first().unwrap()).unwrap();
	let sup_open = if html {
		HTML_SUP_OPEN.to_string()
	} else {
		String::new()
	};
	let sup_close = if html {
		HTML_SUP_CLOSE.to_string()
	} else {
		String::new()
	};
	let mut ctn = format!(
		"{}{}{}{}{} {}{}\n{}\n",
		if n != 1 { "\n\n" } else { "" },
		if_html!(HTML_P_OPEN, html),
		// Numéro de la pièce
		if_html!(HTML_BOLD_OPEN, html),
		i18n.fmt(
			"msg_exhibit",
			&[
				("sup_open", Attr::String(sup_open)),
				("sup_close", Attr::String(sup_close)),
				("nb", Attr::U32(n)),
			]
		),
		if_html!(HTML_BOLD_CLOSE, html),
		// Nature de la pièce
		i18n.fmt(
			"msg_directory",
			&[
				("dir_name", Attr::String(dir_name.display().to_string())),
				("nb", Attr::Usize(files.len())),
				(
					"nb_str",
					Attr::String(nb_repr.usize_to_string(files.len(), i18n))
				),
			]
		),
		if_html!(HTML_P_CLOSE, html),
		// Début de la liste
		if_html!(HTML_LIST_OPEN, html),
	);
	for file in files {
		if let Some(hash) = file.get_hash() {
			let file_size = file.get_size();
			let _ = writeln!(
				ctn,
				"{}{}« {} »{}\n{}{}{}{}{} {}, {} : {}{}{}{}{}",
				// Nom de la pièce
				if !html { " - " } else { "" },
				if_html!(HTML_LIST_ELEM_OPEN, html),
				get_file_name_no_dir(file, &dir_name),
				if_html!(HTML_NEW_LINE, html),
				// Détails de la pièce
				if !html { "   " } else { "" },
				if_html!(HTML_FONT_SMALL_OPEN, html),
				// Taille de la pièce
				if_html!(HTML_ITALIC_OPEN, html),
				file_size,
				if_html!(HTML_ITALIC_CLOSE, html),
				i18n.fmt("msg_file_unit", &[("nb", Attr::U64(file_size))]),
				// Empreinte de la pièce
				TPL_HASH_METHOD,
				if_html!(HTML_ITALIC_OPEN, html),
				hash,
				if_html!(HTML_ITALIC_CLOSE, html),
				if_html!(HTML_FONT_SMALL_CLOSE, html),
				if_html!(HTML_LIST_ELEM_CLOSE, html),
			);
		}
	}
	// Fin de la liste
	let _ = write!(ctn, "{}", if_html!(HTML_LIST_CLOSE, html));
	ctn
}

fn get_clipboard_content(
	i18n: &I18n,
	file_list: &[File],
	html: bool,
	nb_start: u32,
	nb_repr: NbRepr,
) -> String {
	let mut ctn = String::new();
	let mut n = nb_start;
	for e in get_exhibits(file_list) {
		match e {
			Exhibit::Dir(d) => {
				ctn += &format_dir(i18n, n, &d, html, nb_repr);
			}
			Exhibit::File(f) => {
				ctn += &format_file(i18n, n, &f, html);
			}
		};
		n += 1;
	}
	ctn
}

fn get_clipboard_content_ctn_file(
	i18n: &I18n,
	file: &File,
	html: bool,
	nb_files: usize,
	nb_start: u32,
	nb_repr: NbRepr,
) -> String {
	if let Some(hash) = file.get_hash() {
		let sup_open = if html {
			HTML_SUP_OPEN.to_string()
		} else {
			String::new()
		};
		let sup_close = if html {
			HTML_SUP_CLOSE.to_string()
		} else {
			String::new()
		};
		let file_size = file.get_size();
		format!(
			"{}{}{}{} {}{}\n{}{}{}{} {}, {} {}{}{}{}{}",
			if_html!(HTML_P_OPEN, html),
			// Numéro de la pièce
			if_html!(HTML_BOLD_OPEN, html),
			i18n.fmt(
				"msg_exhibit",
				&[
					("sup_open", Attr::String(sup_open)),
					("sup_close", Attr::String(sup_close)),
					("nb", Attr::U32(nb_start)),
				]
			),
			if_html!(HTML_BOLD_CLOSE, html),
			// Nature de la pièce
			i18n.fmt(
				"msg_ctn_file",
				&[
					("file_name", Attr::String(file.display_file_name())),
					("nb", Attr::Usize(nb_files)),
					(
						"nb_str",
						Attr::String(nb_repr.usize_to_string(nb_files, i18n))
					),
					("hash_func", Attr::String(TPL_HASH_METHOD.to_string())),
				]
			),
			if_html!(HTML_NEW_LINE, html),
			if_html!(HTML_FONT_SMALL_OPEN, html),
			// Taille de la pièce
			if_html!(HTML_ITALIC_OPEN, html),
			file_size,
			if_html!(HTML_ITALIC_CLOSE, html),
			i18n.fmt("msg_file_unit", &[("nb", Attr::U64(file_size))]),
			// Empreinte de la pièce
			TPL_HASH_METHOD,
			if_html!(HTML_ITALIC_OPEN, html),
			hash,
			if_html!(HTML_ITALIC_CLOSE, html),
			if_html!(HTML_FONT_SMALL_CLOSE, html),
			if_html!(HTML_P_CLOSE, html),
		)
	} else {
		String::new()
	}
}
