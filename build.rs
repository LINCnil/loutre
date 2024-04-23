#[cfg(windows)]
const FAVICON_NIGHTLY: &str = "assets/favicon-nightly.ico";
#[cfg(windows)]
const FAVICON_RELEASE: &str = "assets/favicon.ico";

#[cfg(windows)]
fn main() {
	let favicon = match std::env::var("PROFILE") {
		Ok(profile) => {
			if profile == "release" {
				FAVICON_RELEASE
			} else {
				FAVICON_NIGHTLY
			}
		}
		Err(_) => FAVICON_NIGHTLY,
	};
	let mut res = winres::WindowsResource::new();
	res.set_icon(favicon);
	res.set_language(0x000c); // French
	res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
