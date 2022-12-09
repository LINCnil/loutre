use crate::checker::check_files;
use crate::clipboard::Clipboard;
use crate::config::Config;
use crate::email::Email;
use crate::file_list::{FileAskAnswer, FileList, FileListBuilder};
use crate::hasher::{FileHasher, HashStatus};
use crate::i18n::{Attr, I18n};
use eframe::egui::{self, Color32, Context, RichText};
use egui_extras::RetainedImage;
use humansize::{make_format, DECIMAL};
use std::path::Path;

const BTN_CLIPBOARD: &str = "🗐";
const BTN_CLIPBOARD_CTN_FILE: &str = "📋";
const BTN_SELECT_DIR: &str = "🗁";
const BTN_SELECT_MAIL: &str = "📧";
const BTN_TRASH: &str = "🗑";
const SIGN_INFO: &str = "ℹ";
const SIGN_SUCCESS: &str = "✔";
const SIGN_WARNING: &str = "⚠";
const UI_EXTRA_SPACE: f32 = 6.0;
const UI_BTN_PADDING_H: f32 = 10.0;
const UI_BTN_PADDING_V: f32 = 6.0;

macro_rules! reset_messages {
	($o: ident) => {
		$o.info_msg = None;
		$o.success_msg = None;
		$o.error_msg = None;
	};
}

pub struct ChecksumApp {
	i18n: I18n,
	clipboard: Clipboard,
	logo: RetainedImage,
	content_file_name: String,
	nb_start: u32,
	file_hasher: Option<FileHasher>,
	file_list: Option<FileList>,
	file_list_builder: Option<FileListBuilder>,
	error_msg: Option<String>,
	success_msg: Option<String>,
	info_msg: Option<String>,
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
	pub fn new(config: &Config) -> Self {
		let logo = RetainedImage::from_image_bytes("logo", &config.theme.get_logo_bytes()).unwrap();
		let i18n = I18n::from_language_tag(&config.lang);
		let content_file_name = config.content_file_name(&i18n);
		Self {
			i18n,
			clipboard: Clipboard::new(),
			logo,
			content_file_name,
			nb_start: crate::NB_FILES_START,
			file_hasher: None,
			file_list: None,
			file_list_builder: None,
			error_msg: None,
			success_msg: None,
			info_msg: None,
			email: None,
		}
	}

	fn update_status(&mut self, ctx: &Context) {
		if let Some(flb) = &mut self.file_list_builder {
			flb.update_state(&self.i18n);
			if flb.is_ready() {
				match flb.get_file_list(&self.i18n) {
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
							self.error_msg = Some(self.i18n.msg("msg_err_fl_not_found"));
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
								match check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.email,
								) {
									Ok(_) => {
										self.success_msg = Some(self.i18n.msg("msg_info_check_ok"));
									}
									Err(e) => {
										self.error_msg = Some(e);
									}
								}
							} else if let Err(e) = fl.write_content_file(&self.i18n) {
								self.error_msg = Some(e.to_string());
							} else if self.email.is_some() {
								match check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.email,
								) {
									Ok(_) => {
										self.success_msg = Some(self.i18n.msg("msg_info_check_ok"));
									}
									Err(e) => {
										self.error_msg = Some(e);
									}
								}
							}
							self.file_hasher = None;
							fl.set_clipboard(&self.i18n, &mut self.clipboard, self.nb_start);
						}
						None => {
							self.error_msg = Some(self.i18n.msg("msg_err_fl_not_found"));
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
					let msg = self.i18n.msg("msg_err_load_dir");
					self.error_msg = Some(self.i18n.fmt(
						"error_desc",
						&[
							("error", Attr::String(e.to_string())),
							("description", Attr::String(msg)),
						],
					));
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
						if ui.button(self.i18n.msg("btn_check_fingerprints")).clicked() {
							reset_messages!(self);
							self.file_hasher = Some(FileHasher::new(p));
						}
					} else if ui.button(self.i18n.msg("btn_calc_fingerprints")).clicked() {
						reset_messages!(self);
						if let Err(e) = p.set_readonly() {
							self.error_msg = Some(e.to_string());
						}
						self.file_hasher = Some(FileHasher::new(p));
					}
					if p.has_hashes()
						&& ui
							.button(BTN_CLIPBOARD)
							.on_hover_text(self.i18n.msg("btn_clipboard_tip"))
							.clicked()
					{
						p.set_clipboard(&self.i18n, &mut self.clipboard, self.nb_start);
					}
					if p.has_hashes()
						&& p.has_content_file() && ui
						.button(BTN_CLIPBOARD_CTN_FILE)
						.on_hover_text(self.i18n.msg("btn_clipboard_ctn_file_tip"))
						.clicked()
					{
						p.set_clipboard_ctn_file(&self.i18n, &mut self.clipboard, self.nb_start);
					}
					ret = true;
				}
			}
		});
		ret
	}

	fn add_file_selection(&mut self, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			if ui
				.button(self.i18n.fmt(
					"btn_select_dir",
					&[("icon", Attr::String(BTN_SELECT_DIR.to_string()))],
				))
				.clicked()
			{
				reset_messages!(self);
				if let Some(path) = rfd::FileDialog::new().pick_folder() {
					self.build_file_list(&path);
				}
			}
			if let Some(p) = &self.file_list {
				if ui
					.button(BTN_TRASH)
					.on_hover_text(self.i18n.msg("btn_trash_tip"))
					.clicked()
				{
					reset_messages!(self);
					self.file_hasher = None;
					self.file_list = None;
				} else {
					ui.add(egui::Label::new(p.to_string()).wrap(true));
				}
			}
		});
		ui.horizontal(|ui| {
			if ui
				.button(self.i18n.fmt(
					"btn_select_mail",
					&[("icon", Attr::String(BTN_SELECT_MAIL.to_string()))],
				))
				.clicked()
			{
				reset_messages!(self);
				if let Some(path) = rfd::FileDialog::new()
					.add_filter(&self.i18n.msg("label_email"), &["msg"])
					.pick_file()
				{
					if let Ok(email) = Email::new(&path) {
						self.email = Some(email);
					}
				}
			}
			if let Some(e) = &self.email {
				if ui
					.button(BTN_TRASH)
					.on_hover_text(self.i18n.msg("btn_trash_tip"))
					.clicked()
				{
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
					ui.label(self.i18n.msg("label_content_file"));
					ui.add(
						egui::TextEdit::singleline(&mut self.content_file_name)
							.interactive(self.file_list.is_none())
							.desired_width(200.0),
					);
					ui.end_row();

					ui.label(self.i18n.msg("label_nb_files_start"));
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
					let file_name = match af.path.file_name() {
						Some(name) => Path::new(name).display(),
						None => af.path.display(),
					};
					let file_name = format!("{}", file_name);
					let msg = if af.is_hidden {
						if af.path.is_dir() {
							self.i18n.fmt(
								"msg_file_choice_dir_hidden",
								&[("file_name", Attr::String(file_name))],
							)
						} else {
							self.i18n.fmt(
								"msg_file_choice_file_hidden",
								&[("file_name", Attr::String(file_name))],
							)
						}
					} else if af.path.is_dir() {
						self.i18n.fmt(
							"msg_file_choice_dir_system",
							&[("file_name", Attr::String(file_name))],
						)
					} else {
						self.i18n.fmt(
							"msg_file_choice_file_system",
							&[("file_name", Attr::String(file_name))],
						)
					};
					ui.label(self.i18n.fmt(
						"msg_file_choice_include",
						&[("file_desc", Attr::String(msg))],
					));
				});
				ui.horizontal(|ui| {
					if ui.button(self.i18n.msg("btn_file_choice.yes")).clicked() {
						flb.answer(FileAskAnswer::Allow);
					}
					if ui
						.button(self.i18n.msg("btn_file_choice.yes_all"))
						.clicked()
					{
						flb.answer(FileAskAnswer::AllowAll);
					}
					if ui.button(self.i18n.msg("btn_file_choice.no")).clicked() {
						flb.answer(FileAskAnswer::Deny);
					}
					if ui.button(self.i18n.msg("btn_file_choice.no_all")).clicked() {
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
			let remaining = self.i18n.fmt(
				"progress",
				&[
					("done", Attr::String(formatter(hr.get_processed_bytes()))),
					("total", Attr::String(formatter(hr.get_total_bytes()))),
				],
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
						&self.i18n.fmt(
							"msg_info_has_ctn_file",
							&[("file_name", Attr::String(self.content_file_name.clone()))],
						),
						|ui| {
							if ui.link(self.i18n.msg("msg_info_del_ctn_file")).clicked() {
								let _ = std::fs::remove_file(p.get_content_file_path());
							}
						},
					);
				} else {
					let nb_files = p.get_nb_files();
					if nb_files >= crate::NB_FILES_WARN_THRESHOLD {
						ChecksumApp::add_warning_label(
							ui,
							&self
								.i18n
								.fmt("msg_info_nb_files", &[("nb", Attr::Usize(nb_files))]),
						);
					}
				}
			}
			if let Some(msg) = &self.info_msg {
				ChecksumApp::add_info_label(ui, msg);
			}
			if let Some(msg) = &self.success_msg {
				ChecksumApp::add_success_label(ui, msg);
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

	fn add_success_label(ui: &mut egui::Ui, text: &str) {
		ChecksumApp::add_label(
			ui,
			text,
			SIGN_SUCCESS,
			&Color32::from_rgb(0xe7, 0xf7, 0xed),
			|_| {},
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
