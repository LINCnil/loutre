use crate::files::HashedFile;
use crate::nb_repr::usize_to_string;
use minijinja::context;
use minijinja::value::Value;
use serde_derive::Serialize;
#[cfg(unix)]
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::{Component, PathBuf};

macro_rules! get_attr_or_ret {
	($ret: ident, $entry: ident, $key: expr) => {
		match $entry.get_attr($key) {
			Ok(value) => value,
			Err(_) => {
				tracing::warn!("unable to get value: {:?}", $entry);
				return $ret;
			}
		}
	};
}

#[cfg(unix)]
macro_rules! get_bytes_or_ret {
	($ret: ident, $entry: ident, $key: expr) => {{
		let value = get_attr_or_ret!($ret, $entry, $key);
		if let minijinja::value::ValueKind::Seq = value.kind() {
			value
				.try_iter()
				.unwrap()
				.map(|e| e.as_usize().unwrap() as u8)
				.collect::<Vec<u8>>()
		} else {
			tracing::warn!("unable to get bytes: {:?}", value);
			return $ret;
		}
	}};
}

#[cfg(not(unix))]
macro_rules! get_string_or_ret {
	($ret: ident, $entry: ident, $key: expr) => {{
		let value = get_attr_or_ret!($ret, $entry, $key);
		if let Some(s) = value.as_str() {
			std::ffi::OsStr::new(s).to_os_string()
		} else {
			tracing::warn!("unable to get string: {:?}", value);
			return $ret;
		}
	}};
}

#[derive(Clone, Debug, Serialize)]
pub struct EntryTemplate {
	#[cfg(unix)]
	pub(crate) base_dir: Vec<u8>,
	#[cfg(not(unix))]
	pub(crate) base_dir: String,
	#[cfg(unix)]
	pub(crate) relative_path: Vec<u8>,
	#[cfg(not(unix))]
	pub(crate) relative_path: String,
	pub(crate) name: String,
	pub(crate) is_dir: bool,
	pub(crate) is_file: bool,
	pub(crate) size: u64,
	pub(crate) hash: String,
	pub(crate) hash_func: String,
	pub(crate) evidences: Vec<EntryTemplate>,
}

impl From<HashedFile> for EntryTemplate {
	fn from(hf: HashedFile) -> Self {
		let base_dir = hf.get_base_dir();
		#[cfg(unix)]
		let base_dir = base_dir.into_os_string().into_vec();
		#[cfg(not(unix))]
		let base_dir = base_dir.to_str().unwrap().to_string();

		let relpath_os = hf.get_relative_path().as_os_str();
		#[cfg(unix)]
		let relative_path = relpath_os.to_os_string().into_vec();
		#[cfg(not(unix))]
		let relative_path = relpath_os.to_str().unwrap().to_string();

		Self {
			base_dir,
			relative_path,
			name: hf.get_relative_path().display().to_string(),
			is_dir: false,
			is_file: true,
			size: hf.get_size(),
			hash: hf.get_hash().to_string(),
			hash_func: hf.get_hash_func().to_string(),
			evidences: Vec::new(),
		}
	}
}

pub fn filter_add_dir_level(lst: Vec<Value>) -> Vec<Value> {
	let mut new_lst = Vec::with_capacity(lst.len());
	let mut last_dir: Option<(PathBuf, Value)> = None;

	for entry in &lst {
		// Get the base directory as a PathBuf.
		#[cfg(not(unix))]
		let base_dir = get_string_or_ret!(lst, entry, "relative_path");
		#[cfg(unix)]
		let base_dir = OsString::from_vec(get_bytes_or_ret!(lst, entry, "base_dir"));
		let mut base_dir = PathBuf::from(base_dir);

		// Get the relative path as a PathBuf.
		#[cfg(not(unix))]
		let relative_path = get_string_or_ret!(lst, entry, "relative_path");
		#[cfg(unix)]
		let relative_path = OsString::from_vec(get_bytes_or_ret!(lst, entry, "relative_path"));
		let relative_path = PathBuf::from(relative_path);

		// Get the components of the relative path.
		let components: Vec<Component> = relative_path
			.components()
			.filter(|e| matches!(e, Component::Normal(_)))
			.collect();

		// Do something only if there is at least one component il the path (ie. the path is not
		// empty).
		if let Some(first_cmp) = components.first() {
			let curr_dir_name = PathBuf::from(first_cmp);

			// If we have previously seen a directory and the current entry is outside of this
			// directory, push the previous directory into the final list.
			if let Some((ref dir_name, ref dir_value)) = last_dir {
				if *dir_name != curr_dir_name {
					new_lst.push(dir_value.clone());
					last_dir = None;
				}
			}

			if components.len() == 1 {
				// We have a file: we push it.
				new_lst.push(entry.clone());
			} else {
				// We have a directory: we prepare the new entry.
				let mut new_relative_path = PathBuf::with_capacity(relative_path.capacity());
				let mut components_it = components.iter();
				if let Some(cpn) = components_it.next() {
					base_dir.push(cpn);
				}
				for cpn in components_it {
					new_relative_path.push(cpn);
				}

				#[cfg(unix)]
				let base_dir = base_dir.into_os_string().into_vec();
				#[cfg(not(unix))]
				let base_dir = base_dir.display().to_string();

				let relpath_os = new_relative_path.as_os_str();
				#[cfg(unix)]
				let relative_path = relpath_os.to_os_string().into_vec();
				#[cfg(not(unix))]
				let relative_path = relpath_os.to_str().unwrap().to_string();

				let new_entry = context!(
					base_dir => base_dir.clone(),
					relative_path,
					name => new_relative_path.display().to_string(),
					is_dir => false,
					is_file => true,
					size => get_attr_or_ret!(lst, entry, "size"),
					hash => get_attr_or_ret!(lst, entry, "hash"),
					hash_func => get_attr_or_ret!(lst, entry, "hash_func"),
					evidences => Vec::<Value>::new(),
				);
				match last_dir {
					Some((_dir_name, last_dir_entry)) => {
						// We have already seen the directory, therefore we have to push the
						// current file into it.
						let entries_val = get_attr_or_ret!(lst, last_dir_entry, "evidences");
						let mut new_entries = Vec::new();
						for e in entries_val.try_iter().unwrap() {
							new_entries.push(e);
						}
						new_entries.push(new_entry);

						let last_dir_value = context!(
							base_dir => get_attr_or_ret!(lst, last_dir_entry, "base_dir"),
							relative_path => get_attr_or_ret!(lst, last_dir_entry, "relative_path"),
							name => get_attr_or_ret!(lst, last_dir_entry, "name"),
							is_dir => true,
							is_file => false,
							size => new_entries.len(),
							hash => String::new(),
							hash_func => String::new(),
							evidences => new_entries,
						);
						last_dir = Some((curr_dir_name, last_dir_value))
					}
					None => {
						// We have not already seen the directory, therefore we have to initialize
						// it.
						#[cfg(unix)]
						let relative_path: Vec<u8> = Vec::new();
						#[cfg(not(unix))]
						let relative_path = String::new();
						let last_dir_value = context!(
							base_dir,
							relative_path,
							name => curr_dir_name.display().to_string(),
							is_dir => true,
							is_file => false,
							size => 1,
							hash => String::new(),
							hash_func => String::new(),
							evidences => vec![new_entry],
						);
						last_dir = Some((curr_dir_name, last_dir_value))
					}
				}
			}
		}
	}
	new_lst
}

pub fn filter_nb_letters(nb: usize) -> String {
	usize_to_string(nb)
}
