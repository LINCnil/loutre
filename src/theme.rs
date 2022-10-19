use std::env;

#[derive(Clone)]
pub enum Theme {
	Dark,
	Light,
}

impl Theme {
	pub fn get_icon_bytes(&self) -> Vec<u8> {
		include_bytes!("../assets/ico/32-32.png").to_vec()
	}

	pub fn get_logo_bytes(&self) -> Vec<u8> {
		match self {
			Theme::Dark => include_bytes!("../assets/cnil-logo-dark.png").to_vec(),
			Theme::Light => include_bytes!("../assets/cnil-logo.png").to_vec(),
		}
	}
}

impl Default for Theme {
	fn default() -> Self {
		let mut theme = Theme::Light;
		let args: Vec<String> = env::args().collect();
		if args.len() == 3 && args[1] == "--theme" {
			theme = match args[2].as_str() {
				"dark" => Theme::Dark,
				_ => Theme::Light,
			}
		}
		theme
	}
}

impl From<Theme> for eframe::Theme {
	fn from(t: Theme) -> Self {
		match t {
			Theme::Dark => eframe::Theme::Dark,
			Theme::Light => eframe::Theme::Light,
		}
	}
}
