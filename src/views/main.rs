use crate::app::ChecksumApp;
use crate::checker::CheckResult;
use crate::file_list::{FileAskAnswer, FileListBuilder};
use crate::hasher::FileHasher;
use crate::i18n::Attr;
use crate::receipt::Receipt;
use crate::theme::{Button, Color, Icon, InfoBox, InfoBoxLevel, InfoBoxType};
use crate::views::AppView;
use eframe::egui::{self, Image};
use humansize::{make_format, DECIMAL};
use std::path::Path;

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let spacing = ui.spacing_mut();
	app.default_padding = spacing.button_padding;
	spacing.button_padding = egui::vec2(super::UI_BTN_PADDING_H, super::UI_BTN_PADDING_V);

	add_header(app, ui);
	ui.add_space(super::UI_EXTRA_SPACE);
	add_file_selection(app, ui);
	ui.add_space(super::UI_EXTRA_SPACE);
	if add_messages(app, ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	if add_loading(app, ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	if add_action_buttons(app, ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	if add_progress_bar(app, ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
}

pub fn handle_dropped_files(app: &mut ChecksumApp, ctx: &egui::Context) {
	for f in &ctx.input(|i| i.raw.dropped_files.clone()) {
		if let Some(path) = &f.path {
			if path.is_dir() {
				build_file_list(app, path);
			}
			if let Ok(receipt) = Receipt::new(path) {
				app.receipt = Some(receipt);
			}
		}
	}
}

fn build_file_list(app: &mut ChecksumApp, path: &Path) {
	if path.is_dir() {
		crate::app::reset_messages!(app);
		app.file_hasher = None;
		app.file_list = None;
		app.file_list_builder = None;
		match FileListBuilder::from_dir(path, &app.content_file_name) {
			Ok(flb) => {
				app.file_list_builder = Some(flb);
			}
			Err(e) => {
				let msg = app.i18n.msg("msg_err_load_dir");
				app.error_msg = Some(app.i18n.fmt(
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

fn add_header(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		let (logo_name, logo_bytes) = app.theme.get_logo_bytes();
		ui.add(Image::from_bytes(logo_name, logo_bytes).fit_to_original_size(1.0));

		ui.vertical(|ui| {
			ui.add_space(super::UI_EXTRA_SPACE);
			ui.add_space(super::UI_EXTRA_SPACE);

			ui.horizontal(|ui| {
				ui.label(app.i18n.msg("label_nb_files_start"));
				let mut nb_str = app.nb_start.to_string();
				let response = ui.add(egui::TextEdit::singleline(&mut nb_str).desired_width(40.0));
				if response.changed() {
					nb_str.retain(|c| c.is_ascii_digit());
					if let Ok(nb) = nb_str.parse::<u32>() {
						app.nb_start = nb.max(1);
					}
				}
			});

			ui.add_space(super::UI_EXTRA_SPACE);

			ui.horizontal(|ui| {
				// Button: select dir
				if ui
					.add(
						Button::new()
							.icon(Icon::ButtonSelectDir)
							.text(app.i18n.msg("btn_select_dir"))
							.render(),
					)
					.clicked()
				{
					crate::app::reset_messages!(app);
					if let Some(path) = rfd::FileDialog::new().pick_folder() {
						build_file_list(app, &path);
					}
				}
				// Button: open receipt
				if ui
					.add(
						Button::new()
							.icon(Icon::ButtonSelectReceipt)
							.text(app.i18n.msg("btn_select_receipt"))
							.render(),
					)
					.clicked()
				{
					crate::app::reset_messages!(app);
					if let Some(path) = rfd::FileDialog::new()
						.add_filter(app.i18n.msg("label_receipt"), &["msg"])
						.pick_file()
					{
						if let Ok(receipt) = Receipt::new(&path) {
							app.receipt = Some(receipt);
						}
					}
				}
				// Button: config
				if ui
					.add_enabled(
						app.file_list.is_none(),
						Button::new()
							.icon(Icon::ButtonConfig)
							.text(app.i18n.msg("config"))
							.render(),
					)
					.on_disabled_hover_text(app.i18n.msg("config_not_available"))
					.clicked()
				{
					app.view = AppView::Config;
				}
			});
		});
	});
}

fn add_file_selection(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		if let Some(fl) = &app.file_list {
			let p = fl.to_string();
			ui.visuals_mut().override_text_color = Some(Color::FileSelection.get(app.theme));
			egui::Frame::none()
				.inner_margin(egui::Margin::from(crate::theme::LARGE_PADDING))
				.rounding(crate::theme::MAIN_ROUNDING)
				.fill(Color::FileSelectionBackground.get(app.theme))
				.show(ui, |ui| {
					ui.horizontal(|ui| {
						ui.label(Icon::ButtonSelectDir.to_string());
						ui.label(p);
						if ui
							.link(Icon::ButtonTrash.to_string())
							.on_hover_text(app.i18n.msg("btn_trash_tip"))
							.clicked()
						{
							crate::app::reset_messages!(app);
							app.file_hasher = None;
							app.file_list = None;
						}
					});
				});
		}
	});
	ui.horizontal(|ui| {
		if let Some(e) = &app.receipt {
			let e = e.to_string();
			ui.visuals_mut().override_text_color = Some(Color::FileSelection.get(app.theme));
			egui::Frame::none()
				.inner_margin(egui::Margin::from(crate::theme::LARGE_PADDING))
				.rounding(crate::theme::MAIN_ROUNDING)
				.fill(Color::FileSelectionBackground.get(app.theme))
				.show(ui, |ui| {
					ui.horizontal(|ui| {
						ui.label(Icon::ButtonSelectReceipt.to_string());
						ui.label(e);
						if ui
							.link(Icon::ButtonTrash.to_string())
							.on_hover_text(app.i18n.msg("btn_trash_tip"))
							.clicked()
						{
							app.receipt = None;
						}
					});
				});
		}
	});
}

fn add_messages(app: &mut ChecksumApp, ui: &mut egui::Ui) -> bool {
	let mut has_messages = false;
	egui::ScrollArea::vertical().show(ui, |ui| {
		if let Some(p) = &app.file_list {
			if p.has_content_file() {
				InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Info).render_dyn(
					ui,
					|ui| {
						ui.label(&app.i18n.fmt(
							"msg_info_has_ctn_file",
							&[("file_name", Attr::String(app.content_file_name.clone()))],
						));
					},
				);
				has_messages = true;
			}
		}
		if let Some(msg) = &app.info_msg {
			InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Info).render_text(ui, msg);
			has_messages = true;
		}
		if let Some(msg) = &app.success_msg {
			InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Success).render_text(ui, msg);
			has_messages = true;
		}
		if let Some(msg) = &app.error_msg {
			InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Warning).render_text(ui, msg);
			has_messages = true;
		}
		if let Some(p) = &app.file_list {
			for fl in p.iter_duplicate_hashes() {
				let mut msg = app.i18n.msg("msg_info_duplicate_hash");
				for f in fl {
					msg += &format!("\n - {}", f.get_path().display());
				}
				InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Warning)
					.render_text(ui, &msg);
			}
			for f in p.iter_empty_files() {
				InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Warning).render_text(
					ui,
					&app.i18n.fmt(
						"msg_info_empty_file",
						&[(
							"file_name",
							Attr::String(f.get_path().display().to_string()),
						)],
					),
				);
			}
		}
	});
	has_messages
}

fn add_loading(app: &mut ChecksumApp, ui: &mut egui::Ui) -> bool {
	if let Some(flb) = &mut app.file_list_builder {
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
						app.i18n.fmt(
							"msg_file_choice_dir_hidden",
							&[("file_name", Attr::String(file_name))],
						)
					} else {
						app.i18n.fmt(
							"msg_file_choice_file_hidden",
							&[("file_name", Attr::String(file_name))],
						)
					}
				} else if af.path.is_dir() {
					app.i18n.fmt(
						"msg_file_choice_dir_system",
						&[("file_name", Attr::String(file_name))],
					)
				} else {
					app.i18n.fmt(
						"msg_file_choice_file_system",
						&[("file_name", Attr::String(file_name))],
					)
				};
				ui.label(app.i18n.fmt(
					"msg_file_choice_include",
					&[("file_desc", Attr::String(msg))],
				));
			});
			ui.horizontal(|ui| {
				if ui
					.add(
						Button::new()
							.text(app.i18n.msg("btn_file_choice.yes"))
							.render(),
					)
					.clicked()
				{
					flb.answer(FileAskAnswer::Allow);
				}
				if ui
					.add(
						Button::new()
							.text(app.i18n.msg("btn_file_choice.yes_all"))
							.render(),
					)
					.clicked()
				{
					flb.answer(FileAskAnswer::AllowAll);
				}
				if ui
					.add(
						Button::new()
							.text(app.i18n.msg("btn_file_choice.no"))
							.render(),
					)
					.clicked()
				{
					flb.answer(FileAskAnswer::Deny);
				}
				if ui
					.add(
						Button::new()
							.text(app.i18n.msg("btn_file_choice.no_all"))
							.render(),
					)
					.clicked()
				{
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

fn add_action_buttons(app: &mut ChecksumApp, ui: &mut egui::Ui) -> bool {
	let mut ret = false;
	ui.horizontal(|ui| {
		if app.file_hasher.is_none() {
			if let Some(p) = &mut app.file_list {
				if p.has_content_file() {
					if ui
						.add(
							Button::new()
								.text(app.i18n.msg("btn_check_fingerprints"))
								.render(),
						)
						.clicked()
					{
						crate::app::reset_messages!(app);
						app.file_hasher = Some(FileHasher::new(p, app.hash));
					}
				} else if ui
					.add(
						Button::new()
							.text(app.i18n.msg("btn_calc_fingerprints"))
							.render(),
					)
					.clicked()
				{
					crate::app::reset_messages!(app);
					if let Err(e) = p.set_readonly() {
						app.error_msg = Some(e.to_string());
					}
					app.file_hasher = Some(FileHasher::new(p, app.hash));
				}
				if p.has_hashes()
					&& p.has_content_file()
					&& app.file_check_result.as_ref().is_some_and(|e| e.is_of())
					&& ui
						.add(Button::new().icon(Icon::ButtonClipboard).render())
						.on_hover_text(app.i18n.msg("btn_clipboard_tip"))
						.clicked()
				{
					p.set_clipboard(&app.i18n, &mut app.clipboard, app.nb_start);
				}
				if p.has_hashes()
					&& p.has_content_file()
					&& app.file_check_result.as_ref().is_some_and(|e| e.is_of())
					&& ui
						.add(
							Button::new()
								.icon(Icon::ButtonClipboardContentFile)
								.render(),
						)
						.on_hover_text(app.i18n.msg("btn_clipboard_ctn_file_tip"))
						.clicked()
				{
					p.set_clipboard_ctn_file(&app.i18n, &mut app.clipboard, app.nb_start, app.hash);
				}
				if let Some(result) = &app.file_check_result {
					match result {
						CheckResult::Success(_) => {
							InfoBox::new(app.theme, InfoBoxType::Simple, InfoBoxLevel::Success)
								.render_text(ui, app.i18n.msg("msg_info_check_ok"));
						}
						CheckResult::CheckErrors(_) => {
							if ui
								.add(Button::new().text(app.i18n.msg("view_errors")).render())
								.clicked()
							{
								app.view = AppView::CheckErrors;
							}
							InfoBox::new(app.theme, InfoBoxType::Simple, InfoBoxLevel::Error)
								.render_text(ui, app.i18n.msg("msg_info_check_error"));
						}
						CheckResult::OtherError(s) => {
							InfoBox::new(app.theme, InfoBoxType::Simple, InfoBoxLevel::Error)
								.render_text(ui, s);
						}
					}
				}
				ret = true;
			}
		}
	});
	ret
}

fn add_progress_bar(app: &mut ChecksumApp, ui: &mut egui::Ui) -> bool {
	if let Some(hr) = &app.file_hasher {
		let progress_bar = egui::ProgressBar::new(hr.get_progress())
			.show_percentage()
			.animate(true);
		ui.add(progress_bar);
		let formatter = make_format(DECIMAL);
		let remaining = app.i18n.fmt(
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
