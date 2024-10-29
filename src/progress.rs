#[derive(Clone, Copy, Debug)]
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
	max: usize,
	value: usize,
}

impl ProgressBarStatus {
	pub fn new(max: usize) -> Self {
		Self { max, value: 0 }
	}

	pub fn get_max(&self) -> usize {
		self.max
	}

	pub fn get_value(&self) -> usize {
		self.value
	}

	pub fn add_progress(&mut self, progress: usize) {
		self.value += progress;
	}
}
