fn is_nightly() -> bool {
	match std::env::var("PROFILE") {
		Ok(profile) => {
			if profile != "release" {
				return true;
			}
			std::env::var("CARGO_FEATURE_NIGHTLY").is_ok()
		}
		Err(_) => true,
	}
}

#[cfg(windows)]
fn get_favicon_path() -> &'static str {
	if is_nightly() {
		"assets/favicon-nightly.ico"
	} else {
		"assets/favicon.ico"
	}
}

#[cfg(windows)]
fn set_windows_metadata() {
	let mut res = winres::WindowsResource::new();
	res.set_icon(get_favicon_path());
	res.set_language(0x000c); // French
	res.compile().unwrap();
}

fn main() {
	if is_nightly() {
		println!("cargo:rustc-cfg=feature=\"nightly\"");
	}
	#[cfg(windows)]
	set_windows_metadata();
}
