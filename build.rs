use base64::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

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
	let mut res = winresource::WindowsResource::new();
	res.set_icon(get_favicon_path());
	res.compile().unwrap();
}

fn push_font_content<P: AsRef<Path>>(
	src: P,
	dest: &mut File,
	font_family: &str,
	font_format: &str,
) {
	let raw_data: &[u8] = &std::fs::read(src.as_ref()).unwrap();
	let b64_str = BASE64_STANDARD.encode(raw_data);
	let font_type = format!("font/{font_format}");
	let url = format!("data:{font_type};base64,{b64_str}");
	let css_str = format!("\n@font-face {{ font-family: \"{font_family}\"; src: url({url}) format(\"{font_format}\"); }}\n");
	let _ = dest.write(css_str.as_bytes()).unwrap();
}

fn push_file_content<P: AsRef<Path>>(src: P, dest: &mut File) {
	let src = src.as_ref();
	let mut reader: &[u8] = &std::fs::read(src)
		.map_err(|e| format!("{}: {e}", src.display()))
		.unwrap();
	std::io::copy(&mut reader, dest).unwrap();
}

fn build_css(dest_path: &str) {
	// Create the final CSS file.
	let mut dest = PathBuf::from(env::var("OUT_DIR").unwrap());
	dest.push(dest_path);
	let mut dest_file = File::create(dest).expect("unable to create {dest_path}");

	// Push the fonts.
	push_file_content("assets/fonts/remixicon.css", &mut dest_file);
	push_font_content(
		"assets/fonts/OpenSans.woff2",
		&mut dest_file,
		"Open Sans",
		"woff2",
	);
	push_font_content(
		"assets/fonts/remixicon.woff2",
		&mut dest_file,
		"remixicon",
		"woff2",
	);

	// Push the base style.
	push_file_content("assets/style/colors.css", &mut dest_file);
	push_file_content("assets/style/main.css", &mut dest_file);

	// Push the style of every component.
	for entry in std::fs::read_dir("assets/style/components").unwrap() {
		push_file_content(entry.unwrap().path(), &mut dest_file);
	}

	// Push the style of every view.
	for entry in std::fs::read_dir("assets/style/views").unwrap() {
		push_file_content(entry.unwrap().path(), &mut dest_file);
	}
}

fn main() {
	if is_nightly() {
		println!("cargo:rustc-cfg=feature=\"nightly\"");
	}
	build_css("loutre.css");
	#[cfg(windows)]
	set_windows_metadata();
}
