use crate::checker::check_files;
use crate::clipboard::Clipboard;
use crate::email::Email;
use crate::file_list::{FileAskAnswer, FileList, FileListBuilder};
use crate::hasher::{FileHasher, HashStatus};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Context, RichText};
use egui_extras::RetainedImage;
use humansize::{make_format, DECIMAL};
use std::path::Path;

const BTN_CLIPBOARD: &str = "🗐";
const BTN_CLIPBOARD_TIP: &str = "Copier l'empreinte de l'ensemble des pièces";
const BTN_CLIPBOARD_CTN_FILE: &str = "📋";
const BTN_CLIPBOARD_CTN_FILE_TIP: &str = "Copier l'empreinte du fichier contenant les empreintes";
const BTN_FILE_CHOICE_YES: &str = "Oui";
const BTN_FILE_CHOICE_YESALL: &str = "Oui pour tous";
const BTN_FILE_CHOICE_NO: &str = "Non";
const BTN_FILE_CHOICE_NOALL: &str = "Non pour tous";
const BTN_SELECT_DIR: &str = "🗁 Ouvrir un dossier…";
const BTN_SELECT_MAIL: &str = "📧 Ouvrir un AR…";
const BTN_TRASH: &str = "🗑";
const BTN_TRASH_TIP: &str = "Réinitialiser";
const BTN_CACL_FINGERPRINTS: &str = "Calculer les empreintes";
const BTN_CHECK_FINGERPRINTS: &str = "Vérifier les empreintes";
const LABEL_NB_FILES_START: &str = "Numéro de la première pièce";
const LABEL_CONTENT_FILE: &str = "Nom du fichier d'empreintes";
const LABEL_EMAIL: &str = "Courrier électronique";
const MSG_ERR_FL_NOT_FOUND: &str = "Erreur interne: liste de fichiers non trouvée.";
const MSG_ERR_LOAD_DIR: &str = "Erreur lors du chargement du dossier";
const MSG_FILE_CHOICE_DIR_HIDDEN: &str = "est un dossier caché.";
const MSG_FILE_CHOICE_DIR_SYSTEM: &str = "est un dossier système.";
const MSG_FILE_CHOICE_FILE_HIDDEN: &str = "est un fichier caché.";
const MSG_FILE_CHOICE_FILE_SYSTEM: &str = "est un fichier système.";
const MSG_FILE_CHOICE_INCLUDE: &str = "Souhaitez-vous l'inclure ?";
const MSG_INFO_CHECK_OK: &str = "Les empreintes correspondent.";
const MSG_INFO_HAS_CTN_FILE: &str = "Le dossier comporte un fichier";
const MSG_INFO_DEL_CTN_FILE: &str = "supprimer";
const MSG_INFO_NB_FILES_1: &str = "Le dossier comporte";
const MSG_INFO_NB_FILES_2: &str = "fichiers.";
const SIGN_INFO: &str = "ℹ";
const SIGN_WARNING: &str = "⚠";
const UI_EXTRA_SPACE: f32 = 6.0;
const UI_BTN_PADDING_H: f32 = 10.0;
const UI_BTN_PADDING_V: f32 = 6.0;

macro_rules! reset_messages {
	($o: ident) => {
		$o.success_msg = None;
		$o.error_msg = None;
	};
}

pub struct ChecksumApp {
	clipboard: Clipboard,
	logo: RetainedImage,
	content_file_name: String,
	nb_start: u32,
	file_hasher: Option<FileHasher>,
	file_list: Option<FileList>,
	file_list_builder: Option<FileListBuilder>,
	error_msg: Option<String>,
	success_msg: Option<String>,
	email: Option<Email>,
}

impl eframe::App for ChecksumApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.spacing_mut().button_padding = egui::vec2(UI_BTN_PADDING_H, UI_BTN_PADDING_V);
			self.update_status(ctx);
			self.add_header(ui);
			ui.add_space(UI_EXTRA_SPACE);
			self.add_file_selection(ui);
			ui.add_space(UI_EXTRA_SPACE);
			if self.add_loading(ui) {
				ui.add_space(UI_EXTRA_SPACE);
			}
			if self.add_action_buttons(ui) {
				ui.add_space(UI_EXTRA_SPACE);
			}
			if self.add_progress_bar(ui) {
				ui.add_space(UI_EXTRA_SPACE);
			}
			self.add_messages(ui);
			self.handle_dropped_files(ctx);
		});
	}
}

impl ChecksumApp {
	pub fn new(theme: &Theme) -> Self {
		let logo = RetainedImage::from_image_bytes("logo", &theme.get_logo_bytes()).unwrap();
		Self {
			clipboard: Clipboard::new(),
			logo,
			content_file_name: crate::CONTENT_FILE_NAME.into(),
			nb_start: crate::NB_FILES_START,
			file_hasher: None,
			file_list: None,
			file_list_builder: None,
			error_msg: None,
			success_msg: None,
			email: None,
		}
	}

	fn update_status(&mut self, ctx: &Context) {
		if let Some(flb) = &mut self.file_list_builder {
			flb.update_state();
			if flb.is_ready() {
				match flb.get_file_list() {
					Ok(fl) => {
						self.file_list = Some(fl);
						self.file_list_builder = None;
					}
					Err(e) => {
						self.error_msg = Some(e);
					}
				};
			} else {
				ctx.request_repaint();
			}
		}
		if let Some(hr) = &mut self.file_hasher {
			match hr.update_status() {
				HashStatus::NewFile(f) => {
					match &mut self.file_list {
						Some(fl) => fl.replace_file(f),
						None => {
							self.error_msg = Some(MSG_ERR_FL_NOT_FOUND.to_string());
						}
					};
					ctx.request_repaint();
				}
				HashStatus::Error(e) => {
					self.error_msg = Some(e);
					ctx.request_repaint();
				}
				HashStatus::Finished => {
					match &mut self.file_list {
						Some(fl) => {
							if fl.has_content_file() {
								match check_files(fl, &self.content_file_name, &self.email) {
									Ok(_) => {
										self.success_msg = Some(MSG_INFO_CHECK_OK.to_string());
									}
									Err(e) => {
										self.error_msg = Some(e);
									}
								}
							} else if let Err(e) = fl.write_content_file() {
								self.error_msg = Some(e.to_string());
							} else if self.email.is_some() {
								match check_files(fl, &self.content_file_name, &self.email) {
									Ok(_) => {
										self.success_msg = Some(MSG_INFO_CHECK_OK.to_string());
									}
									Err(e) => {
										self.error_msg = Some(e);
									}
								}
							}
							self.file_hasher = None;
							fl.set_clipboard(&mut self.clipboard, self.nb_start);
						}
						None => {
							self.error_msg = Some(MSG_ERR_FL_NOT_FOUND.to_string());
						}
					};
					ctx.request_repaint();
				}
				HashStatus::None => {
					ctx.request_repaint();
				}
			}
		}
	}

	fn build_file_list(&mut self, path: &Path) {
		if path.is_dir() {
			reset_messages!(self);
			self.file_hasher = None;
			self.file_list = None;
			self.file_list_builder = None;
			match FileListBuilder::from_dir(path, &self.content_file_name) {
				Ok(flb) => {
					self.file_list_builder = Some(flb);
				}
				Err(e) => {
					let msg = format!("{}: {}", MSG_ERR_LOAD_DIR, e);
					self.error_msg = Some(msg);
				}
			};
		}
	}

	fn handle_dropped_files(&mut self, ctx: &Context) {
		for f in &ctx.input().raw.dropped_files {
			if let Some(path) = &f.path {
				if path.is_dir() {
					self.build_file_list(path);
				}
				if let Ok(email) = Email::new(path) {
					self.email = Some(email);
				}
			}
		}
	}

	fn add_action_buttons(&mut self, ui: &mut egui::Ui) -> bool {
		let mut ret = false;
		ui.horizontal(|ui| {
			if self.file_hasher.is_none() {
				if let Some(p) = &mut self.file_list {
					if p.has_content_file() {
						if ui.button(BTN_CHECK_FINGERPRINTS).clicked() {
							reset_messages!(self);
							self.file_hasher = Some(FileHasher::new(p));
						}
					} else if ui.button(BTN_CACL_FINGERPRINTS).clicked() {
						reset_messages!(self);
						if let Err(e) = p.set_readonly() {
							self.error_msg = Some(e.to_string());
						}
						self.file_hasher = Some(FileHasher::new(p));
					}
					if p.has_hashes()
						&& ui
							.button(BTN_CLIPBOARD)
							.on_hover_text(BTN_CLIPBOARD_TIP)
							.clicked()
					{
						p.set_clipboard(&mut self.clipboard, self.nb_start);
					}
					if p.has_hashes()
						&& p.has_content_file() && ui
						.button(BTN_CLIPBOARD_CTN_FILE)
						.on_hover_text(BTN_CLIPBOARD_CTN_FILE_TIP)
						.clicked()
					{
						p.set_clipboard_ctn_file(&mut self.clipboard, self.nb_start);
					}
					ret = true;
				}
			}
		});
		ret
	}

	fn add_file_selection(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			if ui.button(BTN_SELECT_DIR).clicked() {
				reset_messages!(self);
				if let Some(path) = rfd::FileDialog::new().pick_folder() {
					self.build_file_list(&path);
				}
			}
			if let Some(p) = &self.file_list {
				if ui.button(BTN_TRASH).on_hover_text(BTN_TRASH_TIP).clicked() {
					reset_messages!(self);
					self.file_hasher = None;
					self.file_list = None;
				} else {
					ui.add(egui::Label::new(p.to_string()).wrap(true));
				}
			}
		});
		ui.horizontal(|ui| {
			if ui.button(BTN_SELECT_MAIL).clicked() {
				reset_messages!(self);
				if let Some(path) = rfd::FileDialog::new()
					.add_filter(LABEL_EMAIL, &["msg"])
					.pick_file()
				{
					if let Ok(email) = Email::new(&path) {
						self.email = Some(email);
					}
				}
			}
			if let Some(e) = &self.email {
				if ui.button(BTN_TRASH).on_hover_text(BTN_TRASH_TIP).clicked() {
					self.email = None;
				} else {
					ui.add(egui::Label::new(e.to_string()).wrap(true));
				}
			}
		});
	}

	fn add_header(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			self.logo.show(ui);

			egui::Grid::new("header_grid")
				.num_columns(2)
				.show(ui, |ui| {
					ui.label(LABEL_CONTENT_FILE);
					ui.add(
						egui::TextEdit::singleline(&mut self.content_file_name)
							.interactive(self.file_list.is_none())
							.desired_width(200.0),
					);
					ui.end_row();

					ui.label(LABEL_NB_FILES_START);
					let mut nb_str = self.nb_start.to_string();
					let response =
						ui.add(egui::TextEdit::singleline(&mut nb_str).desired_width(40.0));
					if response.changed() {
						nb_str.retain(|c| c.is_ascii_digit());
						if let Ok(nb) = nb_str.parse::<u32>() {
							self.nb_start = nb.max(1);
						}
					}
					ui.end_row();
				});
		});
	}

	fn add_loading(&mut self, ui: &mut egui::Ui) -> bool {
		if let Some(flb) = &mut self.file_list_builder {
			if let Some(af) = flb.ask_for() {
				ui.horizontal(|ui| {
					ui.add(egui::Spinner::new());
					let msg = if af.is_hidden {
						if af.path.is_dir() {
							MSG_FILE_CHOICE_DIR_HIDDEN
						} else {
							MSG_FILE_CHOICE_FILE_HIDDEN
						}
					} else if af.path.is_dir() {
						MSG_FILE_CHOICE_DIR_SYSTEM
					} else {
						MSG_FILE_CHOICE_FILE_SYSTEM
					};
					let file_name = match af.path.file_name() {
						Some(name) => Path::new(name).display(),
						None => af.path.display(),
					};
					let msg = format!("{} {} {}", file_name, msg, MSG_FILE_CHOICE_INCLUDE);
					ui.label(msg);
				});
				ui.horizontal(|ui| {
					if ui.button(BTN_FILE_CHOICE_YES).clicked() {
						flb.answer(FileAskAnswer::Allow);
					}
					if ui.button(BTN_FILE_CHOICE_YESALL).clicked() {
						flb.answer(FileAskAnswer::AllowAll);
					}
					if ui.button(BTN_FILE_CHOICE_NO).clicked() {
						flb.answer(FileAskAnswer::Deny);
					}
					if ui.button(BTN_FILE_CHOICE_NOALL).clicked() {
						flb.answer(FileAskAnswer::DenyAll);
					}
				});
			} else {
				ui.add(egui::Spinner::new());
			}
			return true;
		}
		false
	}

	fn add_progress_bar(&mut self, ui: &mut egui::Ui) -> bool {
		if let Some(hr) = &self.file_hasher {
			let progress_bar = egui::ProgressBar::new(hr.get_progress())
				.show_percentage()
				.animate(true);
			ui.add(progress_bar);
			let formatter = make_format(DECIMAL);
			let remaining = format!(
				"Réalisé: {} / {}",
				formatter(hr.get_processed_bytes()),
				formatter(hr.get_total_bytes())
			);
			ui.label(remaining);
			return true;
		}
		false
	}

	fn add_messages(&mut self, ui: &mut egui::Ui) {
		egui::ScrollArea::vertical().show(ui, |ui| {
			if let Some(p) = &self.file_list {
				if p.has_content_file() {
					ChecksumApp::add_info_label_extra(
						ui,
						&format!("{} {}.", MSG_INFO_HAS_CTN_FILE, self.content_file_name),
						|ui| {
							if ui.link(MSG_INFO_DEL_CTN_FILE).clicked() {
								let _ = std::fs::remove_file(p.get_content_file_path());
							}
						},
					);
				} else {
					let nb_files = p.get_nb_files();
					if nb_files >= crate::NB_FILES_WARN_THRESHOLD {
						ChecksumApp::add_warning_label(
							ui,
							&format!(
								"{} {} {}",
								MSG_INFO_NB_FILES_1, nb_files, MSG_INFO_NB_FILES_2
							),
						);
					}
				}
			}
			if let Some(msg) = &self.success_msg {
				ChecksumApp::add_info_label(ui, msg);
			}
			if let Some(msg) = &self.error_msg {
				ChecksumApp::add_warning_label(ui, msg);
			}
		});
	}

	fn add_label<F>(ui: &mut egui::Ui, text: &str, icon: &str, color: &Color32, extra: F)
	where
		F: Fn(&mut egui::Ui),
	{
		let margin = egui::style::Margin::from(6.0);
		egui::Frame::none()
			.inner_margin(margin)
			.fill(*color)
			.show(ui, |ui| {
				ui.horizontal(|ui| {
					ui.label(RichText::new(icon).size(20.0));
					ui.add(egui::Label::new(text).wrap(true));
					extra(ui);
					ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |_ui| {});
				});
			});
	}

	fn add_info_label(ui: &mut egui::Ui, text: &str) {
		ChecksumApp::add_info_label_extra(ui, text, |_| {});
	}

	fn add_info_label_extra<F>(ui: &mut egui::Ui, text: &str, extra: F)
	where
		F: Fn(&mut egui::Ui),
	{
		ChecksumApp::add_label(
			ui,
			text,
			SIGN_INFO,
			&Color32::from_rgb(0x7a, 0xcb, 0xff),
			extra,
		);
	}

	fn add_warning_label(ui: &mut egui::Ui, text: &str) {
		ChecksumApp::add_label(
			ui,
			text,
			SIGN_WARNING,
			&Color32::from_rgb(0xff, 0xeb, 0x3e),
			|_| {},
		);
	}
}
