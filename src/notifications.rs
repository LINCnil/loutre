use std::cmp::Ordering;
use std::collections::HashMap;
use uuid::Uuid;

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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NotificationContext {
	FileList,
	Receipt,
	ComputedHash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notification {
	id: Uuid,
	level: NotificationLevel,
	context: NotificationContext,
	title: String,
	content: String,
	is_html: bool,
}

impl Notification {
	pub fn new(
		level: NotificationLevel,
		context: NotificationContext,
		title: impl Into<String>,
		content: impl Into<String>,
	) -> Self {
		Self {
			id: Uuid::new_v4(),
			level,
			context,
			title: title.into(),
			content: content.into(),
			is_html: false,
		}
	}

	pub fn set_html(&mut self, content: impl Into<String>) {
		self.content = content.into();
		self.is_html = true;
	}

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn get_level(&self) -> NotificationLevel {
		self.level
	}

	pub fn get_title(&self) -> &str {
		self.title.as_str()
	}

	pub fn is_html(&self) -> bool {
		self.is_html
	}
}

impl Ord for Notification {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.level != other.level {
			return self.level.cmp(&other.level);
		}
		if self.title != other.title {
			return self.title.cmp(&other.title);
		}
		self.content.cmp(&other.content)
	}
}

impl PartialOrd for Notification {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl std::fmt::Display for Notification {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.content)
	}
}

#[derive(Clone, Debug)]
pub struct NotificationList(HashMap<Uuid, Notification>);

impl NotificationList {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn insert(&mut self, notification: Notification) -> Option<Notification> {
		self.0.insert(notification.id, notification)
	}

	pub fn remove(&mut self, key: &Uuid) -> Option<Notification> {
		self.0.remove(key)
	}

	pub fn clear_context(&mut self, context: NotificationContext) {
		self.0.retain(|_, v| v.context != context)
	}

	pub fn to_vec(&self) -> Vec<Notification> {
		self.0.values().cloned().collect()
	}
}
