use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Icon {
	ButtonClipboard,
	ButtonClipboardContentFile,
	ButtonConfig,
	ButtonSelectDir,
	ButtonSelectReceipt,
	ButtonTrash,
	SignError,
	SignHelp,
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
			Self::ButtonSelectReceipt => '\u{EEEE}',
			Self::ButtonTrash => '\u{EC2A}',
			Self::SignError => '\u{EB96}',
			Self::SignHelp => '\u{F045}',
			Self::SignInfo => '\u{F448}',
			Self::SignSuccess => '\u{EB80}',
			Self::SignWarning => '\u{EA20}',
		}
	}
}

impl fmt::Display for Icon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, " {} ", self.get_char())
	}
}
