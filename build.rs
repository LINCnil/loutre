use base64::prelude::*;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

// BUFF_SIZE must be a multiple of 3 since the data will be encoded in base64 by chucks of this
// size concatenated to each others.
const BUFF_SIZE: usize = 4002;

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

fn file_to_b64(src_path: &str, dest_path: &str, file_type: &str) {
	let mut dest = PathBuf::from(env::var("OUT_DIR").unwrap());
	dest.push(dest_path);
	let mut dest_file = File::create(dest).expect("unable to create {dest_path}");
	let _ = dest_file
		.write(format!("data:{file_type};base64,").as_bytes())
		.unwrap();
	let mut src_file = File::open(src_path).expect("unable to open {src_path}");
	let mut buffer = [0; BUFF_SIZE];
	loop {
		let n = src_file.read(&mut buffer).unwrap();
		if n == 0 {
			break;
		}
		let b64_data = BASE64_STANDARD.encode(&buffer[..n]);
		let _ = dest_file.write(b64_data.as_bytes()).unwrap();
	}
}

fn main() {
	if is_nightly() {
		println!("cargo:rustc-cfg=feature=\"nightly\"");
	}
	file_to_b64(
		"assets/fonts/OpenSans.woff2",
		"OpenSans.woff2.b64",
		"font/truetype",
	);
	file_to_b64(
		"assets/fonts/remixicon.woff2",
		"remixicon.woff2.b64",
		"font/woff2",
	);
	#[cfg(windows)]
	set_windows_metadata();
}
