use crate::checker::{check_files, CheckResult};
use crate::clipboard::Clipboard;
use crate::config::Config;
use crate::file_list::{FileList, FileListBuilder};
use crate::hasher::HashFunc;
use crate::hasher::{FileHasher, HashStatus};
use crate::i18n::I18n;
use crate::receipt::Receipt;
use crate::theme::Theme;
use crate::views::AppView;
use eframe::egui::{self, Context, Image};
use std::collections::HashSet;
use std::path::Path;

macro_rules! reset_messages {
	($o: ident) => {
		$o.info_msg = None;
		$o.success_msg = None;
		$o.error_msg = None;
		$o.file_check_result = None;
	};
}

pub(crate) use reset_messages;

macro_rules! set_msg_info_check_ok {
	($o: ident) => {
		if let Some(CheckResult::Success(files)) = &$o.file_check_result {
			if !files.is_empty() {
				let mut msg = $o.i18n.msg("msg_info_hash_ignored_files");
				msg += "\n";
				msg += &files
					.iter()
					.map(|f| {
						let f = f.components().next().unwrap().as_os_str();
						let f = Path::new(f);
						format!(" - {}", f.display())
					})
					.collect::<HashSet<String>>()
					.into_iter()
					.collect::<Vec<String>>()
					.join("\n");
				$o.info_msg = Some(msg);
			}
		}
	};
}

pub struct ChecksumApp {
	pub i18n: I18n,
	pub clipboard: Clipboard,
	pub clipboard_threshold: usize,
	pub content_file_name: String,
	pub nb_start: u32,
	pub file_check_result: Option<CheckResult>,
	pub file_hasher: Option<FileHasher>,
	pub file_list: Option<FileList>,
	pub file_list_builder: Option<FileListBuilder>,
	pub error_msg: Option<String>,
	pub success_msg: Option<String>,
	pub info_msg: Option<String>,
	pub receipt: Option<Receipt>,
	pub cfg_hash: HashFunc,
	pub hash: HashFunc,
	pub default_padding: egui::Vec2,
	pub theme: Theme,
	pub view: AppView,
	pub enable_duplicate_file_warning: bool,
	pub enable_empty_file_warning: bool,
	pub tmp_config: Option<Config>,
}

impl eframe::App for ChecksumApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui_extras::install_image_loaders(ctx);
		egui::CentralPanel::default()
			.frame(self.theme.get_main_frame())
			.show(ctx, |ui| {
				let style = ui.style_mut();
				self.theme.set_visuals(&mut style.visuals);
				self.theme.set_interaction(&mut style.interaction);
				self.update_status(ctx);

				egui::Frame::none()
					.inner_margin(crate::UI_MARGIN_LARGE)
					.show(ui, |ui| {
						let (logo_name, logo_bytes) = self.theme.get_logo_bytes();
						ui.add(Image::from_bytes(logo_name, logo_bytes).fit_to_original_size(1.0));
					});

				egui::ScrollArea::both().show(ui, |ui| {
					egui::Frame::none()
						.inner_margin(egui::Margin {
							left: crate::UI_MARGIN_LARGE,
							right: crate::UI_MARGIN_LARGE,
							top: crate::UI_MARGIN_NONE,
							bottom: crate::UI_MARGIN_LARGE,
						})
						.show(ui, |ui| {
							let view = self.view.to_owned();
							view.display(self, ui);
							view.handle_dropped_files(self, ctx);
						});
				});
			});
	}
}

impl ChecksumApp {
	pub fn new(config: &Config) -> Self {
		let i18n = I18n::from_language_tag(&config.lang);
		let content_file_name = config.content_file_name(&i18n);
		let clipboard = Clipboard::new(config.number_representation, config.clipboard_persistence);
		Self {
			i18n,
			clipboard,
			clipboard_threshold: config.get_clipboard_threshold(),
			content_file_name,
			nb_start: crate::NB_FILES_START,
			file_check_result: None,
			file_hasher: None,
			file_list: None,
			file_list_builder: None,
			error_msg: None,
			success_msg: None,
			info_msg: None,
			receipt: None,
			cfg_hash: config.hash_function,
			hash: config.hash_function,
			default_padding: egui::Vec2::default(),
			theme: config.theme,
			view: AppView::default(),
			enable_duplicate_file_warning: config.is_duplicate_file_warning_enabled(),
			enable_empty_file_warning: config.is_empty_file_warning_enabled(),
			tmp_config: None,
		}
	}

	pub fn init_theme(self, cc: &eframe::CreationContext<'_>) -> Self {
		self.theme.set_fonts(&cc.egui_ctx);
		self
	}

	fn update_status(&mut self, ctx: &Context) {
		if let Some(flb) = &mut self.file_list_builder {
			flb.update_state(&self.i18n);
			if flb.is_ready() {
				match flb.get_file_list(&self.i18n) {
					Ok(fl) => {
						self.hash = fl.get_session_hash_func(&self.i18n, self.cfg_hash);
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
				HashStatus::NewFiles(f_lst) => {
					for f in f_lst {
						match &mut self.file_list {
							Some(fl) => fl.replace_file(f),
							None => {
								self.error_msg = Some(self.i18n.msg("msg_err_fl_not_found"));
							}
						};
					}
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
								self.file_check_result = Some(check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.receipt,
								));
							} else if let Err(e) = fl.write_content_file(&self.i18n, self.hash) {
								self.error_msg = Some(e.to_string());
							} else if self.receipt.is_some() {
								self.file_check_result = Some(check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.receipt,
								));
							}
							set_msg_info_check_ok!(self);
							self.file_hasher = None;
							fl.set_clipboard_auto(
								&self.i18n,
								&mut self.clipboard,
								self.nb_start,
								self.hash,
								self.clipboard_threshold,
							);
							fl.build_duplicate_hashes();
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
}
