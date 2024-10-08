use crate::app::ChecksumApp;
use crate::checker::CheckResult;
use crate::theme::Button;
use crate::views::AppView;
use eframe::egui;

macro_rules! file_error {
	($app: ident, $ui: ident, $lst: expr, $title: literal) => {
		if !$lst.is_empty() {
			$ui.add_space(crate::UI_MARGIN_MEDIUM);
			$ui.label($app.theme.title(&$app.i18n.msg($title)));
			$ui.add_space(crate::UI_MARGIN_SMALL);
			for f in &$lst {
				$ui.label(f.path.display().to_string());
			}
		}
	};
}

pub fn display(app: &mut ChecksumApp, ui: &mut egui::Ui) {
	let spacing = ui.spacing_mut();
	app.default_padding = spacing.button_padding;
	spacing.button_padding = egui::vec2(crate::UI_BTN_PADDING_H, crate::UI_BTN_PADDING_V);

	if let Some(CheckResult::CheckErrors(err)) = &app.file_check_result {
		file_error!(app, ui, err.invalid_ctn_file, "title_invalid_ctn_file");
		file_error!(app, ui, err.invalid_receipt, "title_invalid_receipt");
		file_error!(app, ui, err.missing_ctn_file, "title_missing_ctn_file");
		file_error!(app, ui, err.missing_receipt, "title_missing_receipt");
	}

	ui.add_space(crate::UI_MARGIN_MEDIUM);
	if ui
		.add(Button::new().text(app.i18n.msg("back")).render())
		.clicked()
	{
		app.view = AppView::Main;
	}
}
