use crate::app::ChecksumApp;
use crate::email::Email;
use crate::i18n::Attr;
use crate::theme::{Icon, InfoBox, InfoBoxLevel, InfoBoxType};
use crate::views::AppView;
use eframe::egui::{self, Image};
use humansize::{make_format, DECIMAL};

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let spacing = ui.spacing_mut();
	app.default_padding = spacing.button_padding;
	spacing.button_padding = egui::vec2(super::UI_BTN_PADDING_H, super::UI_BTN_PADDING_V);

	add_header(app, ui);
	ui.add_space(super::UI_EXTRA_SPACE);
	add_file_selection(app, ui);
	ui.add_space(super::UI_EXTRA_SPACE);
	if app.add_loading(ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	if app.add_action_buttons(ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	if add_progress_bar(app, ui) {
		ui.add_space(super::UI_EXTRA_SPACE);
	}
	add_messages(app, ui);
}

pub fn handle_dropped_files(app: &mut ChecksumApp, ctx: &egui::Context) {
	for f in &ctx.input(|i| i.raw.dropped_files.clone()) {
		if let Some(path) = &f.path {
			if path.is_dir() {
				app.build_file_list(path);
			}
			if let Ok(email) = Email::new(path) {
				app.email = Some(email);
			}
		}
	}
}

fn add_header(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		let (logo_name, logo_bytes) = app.theme.get_logo_bytes();
		ui.add(Image::from_bytes(logo_name, logo_bytes).fit_to_original_size(1.0));

		egui::Grid::new("header_grid")
			.num_columns(2)
			.show(ui, |ui| {
				ui.spacing_mut().button_padding = app.default_padding;

				ui.label("");
				let btn = egui::Button::new(app.theme.icon(Icon::ButtonConfig.get_char()));
				let (enabled, hover_txt) = if app.file_list.is_none() {
					(true, app.i18n.msg("config"))
				} else {
					// FIXME: hover text is not displayed
					(false, app.i18n.msg("config_not_available"))
				};
				if ui
					.add_enabled(enabled, btn)
					.on_hover_text(hover_txt)
					.clicked()
				{
					app.view = AppView::ConfigView;
				}
				ui.end_row();

				ui.label(app.i18n.msg("label_nb_files_start"));
				let mut nb_str = app.nb_start.to_string();
				let response = ui.add(egui::TextEdit::singleline(&mut nb_str).desired_width(40.0));
				if response.changed() {
					nb_str.retain(|c| c.is_ascii_digit());
					if let Ok(nb) = nb_str.parse::<u32>() {
						app.nb_start = nb.max(1);
					}
				}
				ui.end_row();
			});
	});
}

fn add_file_selection(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		if ui
			.button(app.theme.icon_with_txt(
				Icon::ButtonSelectDir.get_char(),
				&app.i18n.msg("btn_select_dir"),
			))
			.clicked()
		{
			crate::app::reset_messages!(app);
			if let Some(path) = rfd::FileDialog::new().pick_folder() {
				app.build_file_list(&path);
			}
		}
		if let Some(p) = &app.file_list {
			if ui
				.button(app.theme.icon(Icon::ButtonTrash.get_char()))
				.on_hover_text(app.i18n.msg("btn_trash_tip"))
				.clicked()
			{
				crate::app::reset_messages!(app);
				app.file_hasher = None;
				app.file_list = None;
			} else {
				ui.add(egui::Label::new(p.to_string()).wrap(true));
			}
		}
	});
	ui.horizontal(|ui| {
		if ui
			.button(app.theme.icon_with_txt(
				Icon::ButtonSelectMail.get_char(),
				&app.i18n.msg("btn_select_mail"),
			))
			.clicked()
		{
			crate::app::reset_messages!(app);
			if let Some(path) = rfd::FileDialog::new()
				.add_filter(app.i18n.msg("label_email"), &["msg"])
				.pick_file()
			{
				if let Ok(email) = Email::new(&path) {
					app.email = Some(email);
				}
			}
		}
		if let Some(e) = &app.email {
			if ui
				.button(app.theme.icon(Icon::ButtonTrash.get_char()))
				.on_hover_text(app.i18n.msg("btn_trash_tip"))
				.clicked()
			{
				app.email = None;
			} else {
				ui.add(egui::Label::new(e.to_string()).wrap(true));
			}
		}
	});
}

fn add_messages(app: &mut ChecksumApp, ui: &mut egui::Ui) {
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
						if ui.link(app.i18n.msg("msg_info_del_ctn_file")).clicked() {
							let _ = std::fs::remove_file(p.get_content_file_path());
						}
					},
				);
			} else {
				let nb_files = p.get_nb_files();
				if nb_files >= crate::NB_FILES_WARN_THRESHOLD {
					InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Warning).render_text(
						ui,
						app.i18n
							.fmt("msg_info_nb_files", &[("nb", Attr::Usize(nb_files))]),
					);
				}
			}
		}
		if let Some(msg) = &app.info_msg {
			InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Info).render_text(ui, msg);
		}
		if let Some(msg) = &app.success_msg {
			InfoBox::new(app.theme, InfoBoxType::Simple, InfoBoxLevel::Success)
				.render_text(ui, msg);
		}
		if let Some(msg) = &app.error_msg {
			InfoBox::new(app.theme, InfoBoxType::Full, InfoBoxLevel::Warning).render_text(ui, msg);
		}
	});
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
