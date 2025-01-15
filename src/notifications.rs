use std::collections::HashSet;

pub type NotificationBlackList = HashSet<String>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NotificationLevel {
	Error,
	Warning,
	Success,
	Info,
}

impl std::fmt::Display for NotificationLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Error => "error",
				Self::Warning => "warning",
				Self::Success => "success",
				Self::Info => "info",
			}
		)
	}
}

impl std::str::FromStr for NotificationLevel {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"error" => Ok(Self::Error),
			"warning" => Ok(Self::Warning),
			"success" => Ok(Self::Success),
			"info" => Ok(Self::Info),
			_ => Err(()),
		}
	}
}
