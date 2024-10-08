use crate::app::ChecksumApp;
use crate::clipboard::{Clipboard, ClipboardPersistence};
use crate::config::Config;
use crate::i18n::I18n;
use crate::theme::{Button, Icon};
use crate::views::AppView;
use eframe::egui;

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let mut new_config = get_config(app);

	ui.add_space(crate::UI_MARGIN_MEDIUM);
	ui.label(app.theme.title(&app.i18n.msg("config_title")));
	ui.add_space(crate::UI_MARGIN_MEDIUM);

	egui::Grid::new("header_grid")
		.num_columns(2)
		.spacing(egui::Vec2 {
			x: crate::UI_MARGIN_NONE,
			y: crate::UI_MARGIN_SMALL,
		})
		.show(ui, |ui| {
			// Content file name
			let default_content_file_name = app.i18n.msg("content_file_name");
			let mut new_content_file_name = new_config
				.content_file_name
				.clone()
				.unwrap_or(default_content_file_name.clone());
			ui.label(app.i18n.msg("label_content_file"));
			ui.add(
				egui::TextEdit::singleline(&mut new_content_file_name).desired_width(f32::INFINITY),
			);
			if new_content_file_name == default_content_file_name {
				new_config.content_file_name = None;
			} else {
				new_config.content_file_name = Some(new_content_file_name);
			}
			ui.end_row();

			// Hash function
			ui.label(app.i18n.msg("label_hash_function"));
			egui::ComboBox::from_id_source("cfg_hash_function")
				.selected_text(new_config.hash_function.to_string())
				.show_ui(ui, |ui| {
					for hash_func in crate::hasher::HASH_FUNCTIONS {
						ui.selectable_value(
							&mut new_config.hash_function,
							*hash_func,
							hash_func.to_string(),
						);
					}
				});
			ui.end_row();

			// Theme
			ui.label(app.i18n.msg("theme"));
			egui::ComboBox::from_id_source("cfg_theme")
				.selected_text(new_config.theme.display(&app.i18n))
				.show_ui(ui, |ui| {
					for t in crate::theme::AVAILABLE_THEMES {
						ui.selectable_value(&mut new_config.theme, *t, t.display(&app.i18n));
					}
				});
			ui.end_row();

			// Language
			ui.label(app.i18n.msg("language"));
			let selected_lang = crate::i18n::AVAILABLE_LANGUAGES
				.iter()
				.find_map(|(c, n)| {
					if *c == new_config.lang {
						Some(n.to_string())
					} else {
						None
					}
				})
				.unwrap();
			egui::ComboBox::from_id_source("cfg_lang")
				.selected_text(selected_lang)
				.show_ui(ui, |ui| {
					for (lang_code, lang_name) in crate::i18n::AVAILABLE_LANGUAGES {
						ui.selectable_value(
							&mut new_config.lang,
							lang_code.to_string(),
							*lang_name,
						);
					}
				});
			ui.end_row();

			// Number representation
			ui.label(app.i18n.msg("number_representation"));
			egui::ComboBox::from_id_source("cfg_number_representation")
				.selected_text(new_config.number_representation.display(&app.i18n))
				.show_ui(ui, |ui| {
					for (nb_repr, nb_repr_name) in crate::nb_repr::AVAILABLE_NB_REPR {
						ui.selectable_value(
							&mut new_config.number_representation,
							*nb_repr,
							app.i18n.msg(nb_repr_name),
						);
					}
				});
			ui.end_row();

			// Duplicate file warning
			ui.label(app.i18n.msg("duplicate_file_warning"));
			let selected_text = if new_config.is_duplicate_file_warning_enabled() {
				app.i18n.msg("activated")
			} else {
				app.i18n.msg("deactivated")
			};
			egui::ComboBox::from_id_source("cfg_duplicate_file_warning")
				.selected_text(selected_text)
				.show_ui(ui, |ui| {
					ui.selectable_value(
						&mut new_config.enable_duplicate_file_warning,
						Some(true),
						app.i18n.msg("activated"),
					);
					ui.selectable_value(
						&mut new_config.enable_duplicate_file_warning,
						Some(false),
						app.i18n.msg("deactivated"),
					);
				});
			ui.end_row();

			// Empty file warning
			ui.label(app.i18n.msg("empty_file_warning"));
			let selected_text = if new_config.is_empty_file_warning_enabled() {
				app.i18n.msg("activated")
			} else {
				app.i18n.msg("deactivated")
			};
			egui::ComboBox::from_id_source("cfg_empty_file_warning")
				.selected_text(selected_text)
				.show_ui(ui, |ui| {
					ui.selectable_value(
						&mut new_config.enable_empty_file_warning,
						Some(true),
						app.i18n.msg("activated"),
					);
					ui.selectable_value(
						&mut new_config.enable_empty_file_warning,
						Some(false),
						app.i18n.msg("deactivated"),
					);
				});
			ui.end_row();

			// Clipboard threshold
			ui.label(app.i18n.msg("clipboard_threshold") + &Icon::SignHelp.to_string())
				.on_hover_text(app.i18n.msg("clipboard_threshold_help"));
			let mut nb_str = app.clipboard_threshold.to_string();
			let response = ui.add(egui::TextEdit::singleline(&mut nb_str).desired_width(40.0));
			if response.changed() {
				nb_str.retain(|c| c.is_ascii_digit());
				if let Ok(nb) = nb_str.parse::<usize>() {
					app.clipboard_threshold = nb.max(1);
				}
			}
			ui.end_row();

			// Clipboard persistence
			ui.label(app.i18n.msg("clipboard_persistence") + &Icon::SignHelp.to_string())
				.on_hover_text(app.i18n.msg("clipboard_persistence_help"));
			let selected: ClipboardPersistence = new_config.clipboard_persistence.into();
			let selected = selected.display(&app.i18n);
			egui::ComboBox::from_id_source("cfg_clipboard_persistence")
				.selected_text(selected)
				.show_ui(ui, |ui| {
					for p in crate::clipboard::AVAILABLE_PERSISTENCES {
						ui.selectable_value(
							&mut new_config.clipboard_persistence,
							(*p).into(),
							p.display(&app.i18n),
						);
					}
				});
			ui.end_row();
		});

	let spacing = ui.spacing_mut();
	app.default_padding = spacing.button_padding;
	spacing.button_padding = egui::vec2(crate::UI_BTN_PADDING_H, crate::UI_BTN_PADDING_V);

	app.tmp_config = Some(new_config.clone());
	ui.add_space(crate::UI_MARGIN_MEDIUM);
	ui.horizontal(|ui| {
		if ui
			.add(Button::new().text(app.i18n.msg("apply")).render())
			.clicked()
		{
			set_config(app);
		}
		if ui
			.add(Button::new().text(app.i18n.msg("cancel")).render())
			.clicked()
		{
			reset_config(app);
		}
	});
}

fn get_config(app: &ChecksumApp) -> Config {
	match &app.tmp_config {
		Some(cfg) => cfg.clone(),
		None => Config {
			theme: app.theme,
			lang: app.i18n.get_lang_tag(),
			number_representation: app.clipboard.get_nb_repr(),
			content_file_name: Some(app.content_file_name.clone()),
			hash_function: app.cfg_hash,
			clipboard_persistence: app.clipboard.get_persistence(),
			clipboard_threshold: Some(app.clipboard_threshold),
			enable_duplicate_file_warning: Some(app.enable_duplicate_file_warning),
			enable_empty_file_warning: Some(app.enable_empty_file_warning),
		},
	}
}

fn reset_config(app: &mut ChecksumApp) {
	app.tmp_config = None;
	app.view = AppView::Main;
}

fn set_config(app: &mut ChecksumApp) {
	if let Some(cfg) = &app.tmp_config {
		app.theme = cfg.theme;
		app.i18n = I18n::from_language_tag(&cfg.lang);
		app.content_file_name = cfg.content_file_name(&app.i18n);
		app.cfg_hash = cfg.hash_function;
		app.hash = cfg.hash_function;
		app.clipboard = Clipboard::new(cfg.number_representation, cfg.clipboard_persistence);
		app.enable_duplicate_file_warning = cfg.is_duplicate_file_warning_enabled();
		app.enable_empty_file_warning = cfg.is_empty_file_warning_enabled();
		cfg.write_to_file();
	}
	reset_config(app)
}
