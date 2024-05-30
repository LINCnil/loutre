use crate::checker::{check_files, CheckResult};
use crate::clipboard::Clipboard;
use crate::config::Config;
use crate::email::Email;
use crate::file_list::{FileList, FileListBuilder};
use crate::hasher::HashFunc;
use crate::hasher::{FileHasher, HashStatus};
use crate::i18n::I18n;
use crate::theme::Theme;
use crate::views::AppView;
use eframe::egui::{self, Context};
use std::collections::HashSet;
use std::path::Path;

macro_rules! reset_messages {
	($o: ident) => {
		$o.info_msg = None;
		$o.success_msg = None;
		$o.error_msg = None;
	};
}

pub(crate) use reset_messages;

macro_rules! set_msg_info_check_ok {
	($o: ident, $files: ident) => {
		let mut msg = $o.i18n.msg("msg_info_hash_done");
		if !$files.is_empty() {
			msg += "\n";
			msg += &$o.i18n.msg("msg_info_hash_ignored_files");
			msg += "\n";
			msg += &$files
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
		}
		$o.info_msg = Some(msg);
	};
}

pub struct ChecksumApp {
	pub i18n: I18n,
	pub clipboard: Clipboard,
	pub content_file_name: String,
	pub nb_start: u32,
	pub file_hasher: Option<FileHasher>,
	pub file_list: Option<FileList>,
	pub file_list_builder: Option<FileListBuilder>,
	pub error_msg: Option<String>,
	pub success_msg: Option<String>,
	pub info_msg: Option<String>,
	pub email: Option<Email>,
	pub cfg_hash: HashFunc,
	pub hash: HashFunc,
	pub default_padding: egui::Vec2,
	pub theme: Theme,
	pub view: AppView,
	pub tmp_config: Option<Config>,
}

impl eframe::App for ChecksumApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui_extras::install_image_loaders(ctx);
		egui::CentralPanel::default()
			.frame(self.theme.get_main_frame())
			.show(ctx, |ui| {
				self.theme.set_visuals(ui.visuals_mut());
				self.update_status(ctx);
				let view = self.view.to_owned();
				view.display(self, ui);
				view.handle_dropped_files(self, ctx);
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
			content_file_name,
			nb_start: crate::NB_FILES_START,
			file_hasher: None,
			file_list: None,
			file_list_builder: None,
			error_msg: None,
			success_msg: None,
			info_msg: None,
			email: None,
			cfg_hash: config.hash_function,
			hash: config.hash_function,
			default_padding: egui::Vec2::default(),
			theme: config.theme,
			view: AppView::default(),
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
							let mut ignored_files = Vec::new();
							if fl.has_content_file() {
								match check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.email,
								) {
									CheckResult::Success(ifl) => {
										ignored_files = ifl;
										self.success_msg = Some(self.i18n.msg("msg_info_check_ok"));
									}
									CheckResult::CheckErrors(fe) => {
										// TODO: implement
										self.error_msg = Some("TODO".to_string())
									}
									CheckResult::OtherError(s) => self.error_msg = Some(s),
								}
							} else if let Err(e) = fl.write_content_file(&self.i18n, self.hash) {
								self.error_msg = Some(e.to_string());
							} else if self.email.is_some() {
								match check_files(
									&self.i18n,
									fl,
									&self.content_file_name,
									&self.email,
								) {
									CheckResult::Success(ifl) => {
										ignored_files = ifl;
										self.success_msg = Some(self.i18n.msg("msg_info_check_ok"));
									}
									CheckResult::CheckErrors(fe) => {
										// TODO: implement
										self.error_msg = Some("TODO".to_string())
									}
									CheckResult::OtherError(s) => self.error_msg = Some(s),
								}
							}
							set_msg_info_check_ok!(self, ignored_files);
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
}
