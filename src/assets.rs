macro_rules! include_asset {
	($file_name: expr) => {
		include_str!(concat!(env!("OUT_DIR"), "/", $file_name, ".b64"))
	};
}

pub const FONT_OPEN_SANS_B64: &str = include_asset!("OpenSans.woff2");
pub const FONT_REMIXICON_B64: &str = include_asset!("remixicon.woff2");
pub const LOGO_DARK_B64: &str = include_asset!("main-logo-dark.png");
pub const LOGO_LIGHT_B64: &str = include_asset!("main-logo-light.png");
