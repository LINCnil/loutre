use std::collections::HashSet;

pub type NotificationBlackList = HashSet<String>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NotificationLevel {
	#[cfg(feature = "nightly")]
	Error,
	Warning,
	#[cfg(feature = "nightly")]
	Success,
	Info,
}

impl std::fmt::Display for NotificationLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				#[cfg(feature = "nightly")]
				Self::Error => "error",
				Self::Warning => "warning",
				#[cfg(feature = "nightly")]
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
			#[cfg(feature = "nightly")]
			"error" => Ok(Self::Error),
			"warning" => Ok(Self::Warning),
			#[cfg(feature = "nightly")]
			"success" => Ok(Self::Success),
			"info" => Ok(Self::Info),
			_ => Err(()),
		}
	}
}
