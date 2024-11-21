#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadingBarStatus {
	Displayed,
	Hidden,
}

impl LoadingBarStatus {
	pub fn is_displayed(&self) -> bool {
		match self {
			Self::Displayed => true,
			Self::Hidden => false,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct ProgressBarStatus {
	max: u64,
	value: u64,
}

impl ProgressBarStatus {
	pub fn new(max: u64) -> Self {
		Self { max, value: 0 }
	}

	pub fn get_max(&self) -> u64 {
		self.max
	}

	pub fn get_value(&self) -> u64 {
		self.value
	}

	pub fn add_progress(&mut self, progress: u64) {
		self.value += progress;
	}
}
