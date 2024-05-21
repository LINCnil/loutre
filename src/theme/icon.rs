#[derive(Clone, Copy, Debug)]
pub enum Icon {
	ButtonClipboard,
	ButtonClipboardContentFile,
	ButtonConfig,
	ButtonSelectDir,
	ButtonSelectMail,
	ButtonTrash,
	SignError,
	SignInfo,
	SignSuccess,
	SignWarning,
}

impl Icon {
	pub fn get_char(&self) -> char {
		match self {
			Self::ButtonClipboard => '\u{EB91}',
			Self::ButtonClipboardContentFile => '\u{ECD3}',
			Self::ButtonConfig => '\u{F0E6}',
			Self::ButtonSelectDir => '\u{ED58}',
			Self::ButtonSelectMail => '\u{EEEE}',
			Self::ButtonTrash => '\u{EC2A}',
			Self::SignError => '\u{EB96}',
			Self::SignInfo => '\u{F449}',
			Self::SignSuccess => '\u{EB81}',
			Self::SignWarning => '\u{EA21}',
		}
	}
}