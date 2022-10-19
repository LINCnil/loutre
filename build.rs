#[cfg(windows)]
fn main() {
	let mut res = winres::WindowsResource::new();
	res.set_icon("assets/favicon.ico");
	res.set_language(0x000c); // French
	res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
