use crate::file::File;
use crate::file_list::FileList;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const CTN_FILE_MSG_1: &str =
	"copie sur support informatique d'un document remis au responsable des lieux, intitulé";
const CTN_FILE_MSG_2: &str = "contenant l'inventaire des";
const CTN_FILE_MSG_3: &str = "pièces numériques copiées durant la mission de contrôle. Pour chaque pièce est précisé son intitulé, sa taille et son empreinte numérique au format SHA256.";
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
const TPL_DIR_1: &str = "copie sur support informatique d'un dossier intitulé";
const TPL_DIR_2: &str = "contenant";
const TPL_DIR_3: &str = "document";
const TPL_START_HTML: &str = "PIÈCE N<sup>o</sup>";
const TPL_START_TXT: &str = "PIÈCE No";
const TPL_FILE: &str = "copie sur support informatique d'un document intitulé";
const TPL_FILE_UNIT: &str = "octets";
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
}

impl Clipboard {
	pub fn new() -> Self {
		Self { internal: None }
	}
}

impl Clipboard {
	pub fn set_clipboard(&mut self, file_list: &FileList, nb_start: u32) {
		let html = get_clipboard_content(file_list, true, nb_start);
		let alt_text = get_clipboard_content(file_list, false, nb_start);
		self.set_html(&html, &alt_text);
	}

	pub fn set_clipboard_ctn_file(&mut self, file: &File, nb_files: usize, nb_start: u32) {
		let html = get_clipboard_content_ctn_file(file, true, nb_files, nb_start);
		let alt_text = get_clipboard_content_ctn_file(file, false, nb_files, nb_start);
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
				if cfg!(unix) {
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

fn get_exhibits(file_list: &FileList) -> Vec<Exhibit> {
	let mut lst = Vec::new();
	for f in file_list.iter_files() {
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

fn format_file(n: u32, file: &File, html: bool) -> String {
	let mut ctn = String::new();
	if let Some(hash) = file.get_hash() {
		if n != 1 {
			let _ = writeln!(ctn, "\n");
		}
		let _ = write!(
			ctn,
			"{}{}{} {} :{} {} « {} »{}\n{}{}{}{} {}, {} {}{}{}{}{}",
			if_html!(HTML_P_OPEN, html),
			// Numéro de la pièce
			if_html!(HTML_BOLD_OPEN, html),
			if html { TPL_START_HTML } else { TPL_START_TXT },
			n,
			if_html!(HTML_BOLD_CLOSE, html),
			// Nature de la pièce
			TPL_FILE,
			// Nom de la pièce
			file.display_file_name(),
			if_html!(HTML_NEW_LINE, html),
			if_html!(HTML_FONT_SMALL_OPEN, html),
			// Taille de la pièce
			if_html!(HTML_ITALIC_OPEN, html),
			file.get_size(),
			if_html!(HTML_ITALIC_CLOSE, html),
			TPL_FILE_UNIT,
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

fn format_dir(n: u32, files: &[File], html: bool) -> String {
	let dir_name = get_dir_name(files.first().unwrap()).unwrap();
	let mut ctn = format!(
		"{}{}{}{} {} :{} {} « {} » {} {} {}{} :{}\n{}\n",
		if n != 1 { "\n\n" } else { "" },
		if_html!(HTML_P_OPEN, html),
		// Numéro de la pièce
		if_html!(HTML_BOLD_OPEN, html),
		if html { TPL_START_HTML } else { TPL_START_TXT },
		n,
		if_html!(HTML_BOLD_CLOSE, html),
		// Nature de la pièce
		TPL_DIR_1,
		// Nom du dossier
		dir_name.display(),
		TPL_DIR_2,
		// Nombre de documents
		files.len(),
		TPL_DIR_3,
		if files.len() != 1 { "s" } else { "" },
		if_html!(HTML_P_CLOSE, html),
		// Début de la liste
		if_html!(HTML_LIST_OPEN, html),
	);
	for file in files {
		if let Some(hash) = file.get_hash() {
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
				file.get_size(),
				if_html!(HTML_ITALIC_CLOSE, html),
				TPL_FILE_UNIT,
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

fn get_clipboard_content(file_list: &FileList, html: bool, nb_start: u32) -> String {
	let mut ctn = String::new();
	let mut n = nb_start;
	for e in get_exhibits(file_list) {
		match e {
			Exhibit::Dir(d) => {
				ctn += &format_dir(n, &d, html);
			}
			Exhibit::File(f) => {
				ctn += &format_file(n, &f, html);
			}
		};
		n += 1;
	}
	ctn
}

fn get_clipboard_content_ctn_file(
	file: &File,
	html: bool,
	nb_files: usize,
	nb_start: u32,
) -> String {
	if let Some(hash) = file.get_hash() {
		format!(
			"{}{}{} {} :{} {} « {} » {} {} {}{}\n{}{}{}{} {}, {} {}{}{}{}{}",
			if_html!(HTML_P_OPEN, html),
			// Numéro de la pièce
			if_html!(HTML_BOLD_OPEN, html),
			if html { TPL_START_HTML } else { TPL_START_TXT },
			nb_start,
			if_html!(HTML_BOLD_CLOSE, html),
			// Nature de la pièce
			CTN_FILE_MSG_1,
			// Nom de la pièce
			file.display_file_name(),
			CTN_FILE_MSG_2,
			nb_files,
			CTN_FILE_MSG_3,
			if_html!(HTML_NEW_LINE, html),
			if_html!(HTML_FONT_SMALL_OPEN, html),
			// Taille de la pièce
			if_html!(HTML_ITALIC_OPEN, html),
			file.get_size(),
			if_html!(HTML_ITALIC_CLOSE, html),
			TPL_FILE_UNIT,
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
