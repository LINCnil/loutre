use crate::app::ChecksumApp;
use crate::clipboard::{Clipboard, ClipboardPersistence};
use crate::config::Config;
use crate::i18n::I18n;
use crate::theme::Theme;
use crate::views::AppView;
use eframe::egui;

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let mut new_config = get_config(app);

	egui::Grid::new("header_grid")
		.num_columns(2)
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
			let default_theme = match &new_config.theme {
				#[cfg(feature = "nightly")]
				Theme::NightlyDark => Theme::Dark,
				#[cfg(feature = "nightly")]
				Theme::NightlyLight => Theme::Light,
				_ => new_config.theme,
			};
			egui::ComboBox::from_id_source("cfg_theme")
				.selected_text(default_theme.display(&app.i18n))
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

			// Clipboard persistence
			ui.label(app.i18n.msg("clipboard_persistence"));
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

	app.tmp_config = Some(new_config.clone());
	ui.horizontal(|ui| {
		if ui.button(app.i18n.msg("apply")).clicked() {
			set_config(app);
		}
		if ui.button(app.i18n.msg("cancel")).clicked() {
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
		},
	}
}

fn reset_config(app: &mut ChecksumApp) {
	app.tmp_config = None;
	app.view = AppView::MainView;
}

fn set_config(app: &mut ChecksumApp) {
	if let Some(cfg) = &app.tmp_config {
		app.theme = get_app_theme(cfg);
		app.i18n = I18n::from_language_tag(&cfg.lang);
		app.content_file_name = cfg.content_file_name(&app.i18n);
		app.cfg_hash = cfg.hash_function;
		app.hash = cfg.hash_function;
		app.clipboard = Clipboard::new(cfg.number_representation, cfg.clipboard_persistence);
		cfg.write_to_file();
	}
	reset_config(app)
}

#[cfg(not(feature = "nightly"))]
fn get_app_theme(cfg: &Config) -> Theme {
	cfg.theme
}

#[cfg(feature = "nightly")]
fn get_app_theme(cfg: &Config) -> Theme {
	if cfg.theme == Theme::Dark {
		Theme::NightlyDark
	} else {
		Theme::NightlyLight
	}
}
