use crate::app::ChecksumApp;
use crate::checker::CheckResult;
use crate::theme::Button;
use crate::views::AppView;
use eframe::egui::{self, Image};

macro_rules! file_error {
	($app: ident, $ui: ident, $lst: expr, $title: literal) => {
		if !$lst.is_empty() {
			$ui.label($app.i18n.msg($title));
			for f in &$lst {
				$ui.label(f.path.display().to_string());
			}
		}
	};
}

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let (logo_name, logo_bytes) = app.theme.get_logo_bytes();

	let spacing = ui.spacing_mut();
	app.default_padding = spacing.button_padding;
	spacing.button_padding = egui::vec2(super::UI_BTN_PADDING_H, super::UI_BTN_PADDING_V);

	ui.add(Image::from_bytes(logo_name, logo_bytes).fit_to_original_size(1.0));
	ui.add_space(super::UI_MARGIN_LARGE);

	if ui
		.add(Button::new().text(app.i18n.msg("back")).render())
		.clicked()
	{
		app.view = AppView::Main;
	}
	if let Some(CheckResult::CheckErrors(err)) = &app.file_check_result {
		file_error!(app, ui, err.invalid_ctn_file, "title_invalid_ctn_file");
		file_error!(app, ui, err.invalid_receipt, "title_invalid_receipt");
		file_error!(app, ui, err.missing_ctn_file, "title_missing_ctn_file");
		file_error!(app, ui, err.missing_receipt, "title_missing_receipt");
	}
}
