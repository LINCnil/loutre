use crate::file::File;
use crate::file_list::FileList;
use blake2::{Blake2b512, Blake2s256};
use blake3::Hasher as Blake3;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::cmp::Ordering;
use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fmt, fs, thread};

macro_rules! exit_update_loop {
	($self: ident, $empty_status: expr) => {
		if $self.files_ready.is_empty() {
			$empty_status
		} else {
			let ret = $self.files_ready.clone();
			$self.files_ready.clear();
			HashStatus::NewFiles(ret)
		}
	};
}

pub const HASH_FUNCTIONS: &[HashFunc] = &[
	HashFunc::Sha256,
	HashFunc::Sha384,
	HashFunc::Sha512,
	HashFunc::Sha3_256,
	HashFunc::Sha3_384,
	HashFunc::Sha3_512,
	HashFunc::Blake2s,
	HashFunc::Blake2b,
	HashFunc::Blake3,
];

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum HashFunc {
	#[serde(rename = "sha-256")]
	#[default]
	Sha256,
	#[serde(rename = "sha-384")]
	Sha384,
	#[serde(rename = "sha-512")]
	Sha512,
	#[serde(rename = "sha3-256")]
	Sha3_256,
	#[serde(rename = "sha3-384")]
	Sha3_384,
	#[serde(rename = "sha3-512")]
	Sha3_512,
	#[serde(rename = "blake2s")]
	Blake2s,
	#[serde(rename = "blake2b")]
	Blake2b,
	#[serde(rename = "blake3")]
	Blake3,
}

impl HashFunc {
	pub fn parse(s: &str) -> Result<Self, String> {
		let s = s.to_ascii_lowercase().replace(['-', ' '], "");
		match s.to_ascii_lowercase().as_str() {
			"sha256" => Ok(HashFunc::Sha256),
			"sha384" => Ok(HashFunc::Sha384),
			"sha512" => Ok(HashFunc::Sha512),
			"sha3256" => Ok(HashFunc::Sha3_256),
			"sha3384" => Ok(HashFunc::Sha3_384),
			"sha3512" => Ok(HashFunc::Sha3_512),
			"blake2s" => Ok(HashFunc::Blake2s),
			"blake2b" => Ok(HashFunc::Blake2b),
			"blake3" => Ok(HashFunc::Blake3),
			_ => Err("Invalid hash function".to_string()),
		}
	}

	fn nb_threads(&self) -> usize {
		match thread::available_parallelism() {
			Ok(nb) => nb.get(),
			Err(_) => 1,
		}
	}
}

impl fmt::Display for HashFunc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			HashFunc::Sha256 => "SHA256",
			HashFunc::Sha384 => "SHA384",
			HashFunc::Sha512 => "SHA512",
			HashFunc::Sha3_256 => "SHA3-256",
			HashFunc::Sha3_384 => "SHA3-384",
			HashFunc::Sha3_512 => "SHA3-512",
			HashFunc::Blake2s => "BLAKE2s",
			HashFunc::Blake2b => "BLAKE2b",
			HashFunc::Blake3 => "BLAKE3",
		};
		write!(f, "{s}")
	}
}

enum InternalHashStatus {
	BytesConsumed(u64),
	NewFile(File),
	Error(String),
}

pub enum HashStatus {
	NewFiles(Vec<File>),
	Error(String),
	Finished,
	None,
}

pub struct FileHasher {
	rx: Receiver<InternalHashStatus>,
	processed_bytes: u64,
	total_bytes: u64,
	files_ready: Vec<File>,
}

impl FileHasher {
	pub fn new(file_list: &FileList, hash: HashFunc) -> Self {
		// Define some kind of metadata
		let total_bytes = file_list.get_total_size();
		let (base_tx, rx) = channel();

		// Generate the shared job list
		let mut file_list: Vec<File> = file_list.iter_files().cloned().collect();
		let nb_files = file_list.len();
		file_list.sort_by(cmp_size);
		let shared_lst = Arc::new(Mutex::new(file_list));

		// Spawn hashing threads on each list
		for _ in 0..hash.nb_threads() {
			let tx = base_tx.clone();
			let jobs = shared_lst.clone();
			thread::spawn(move || loop {
				let mut mut_lst = jobs.lock().unwrap();
				let file = match mut_lst.pop() {
					Some(f) => f,
					None => {
						break;
					}
				};
				std::mem::drop(mut_lst);
				let _ = match hash_file(&file, hash, Some(tx.clone())) {
					Ok(nf) => tx.send(InternalHashStatus::NewFile(nf)),
					Err(e) => {
						let msg = format!("{}: {}", file.get_path().display(), e);
						tx.send(InternalHashStatus::Error(msg))
					}
				};
			});
		}

		// Return the FileHasher
		FileHasher {
			rx,
			processed_bytes: 0,
			total_bytes,
			files_ready: Vec::with_capacity(nb_files),
		}
	}

	pub fn update_status(&mut self) -> HashStatus {
		let respond_after = Instant::now() + Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
		loop {
			match self.rx.try_recv() {
				Ok(rsp) => match rsp {
					InternalHashStatus::BytesConsumed(nb) => {
						self.processed_bytes += nb;
					}
					InternalHashStatus::NewFile(f) => {
						self.files_ready.push(f);
					}
					InternalHashStatus::Error(e) => {
						return HashStatus::Error(e);
					}
				},
				Err(e) => match e {
					TryRecvError::Empty => {
						return exit_update_loop!(self, HashStatus::None);
					}
					TryRecvError::Disconnected => {
						return exit_update_loop!(self, HashStatus::Finished);
					}
				},
			};
			if Instant::now() > respond_after {
				return exit_update_loop!(self, HashStatus::None);
			}
		}
	}

	pub fn get_progress(&self) -> f32 {
		(self.processed_bytes as f32) / (self.total_bytes as f32)
	}

	pub fn get_processed_bytes(&self) -> u64 {
		self.processed_bytes
	}

	pub fn get_total_bytes(&self) -> u64 {
		self.total_bytes
	}
}

fn cmp_size(a: &File, b: &File) -> Ordering {
	a.get_size().cmp(&b.get_size())
}

pub fn hash_single_file(file: &File, hash: HashFunc) -> io::Result<File> {
	hash_file(file, hash, None)
}

macro_rules! alg_hash_file {
	($f: expr, $buffer: expr, $tx: expr, $alg: ident) => {{
		let mut hasher = $alg::new();
		let mut processed_bytes = 0;
		let mut last_notif = Instant::now();
		let ref_duration = Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
		loop {
			let n = $f.read(&mut $buffer)?;
			if n == 0 {
				if let Some(ref s) = $tx {
					let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
				}
				break;
			}
			hasher.update(&$buffer[..n]);
			processed_bytes += n as u64;
			if last_notif.elapsed() >= ref_duration {
				if let Some(ref s) = $tx {
					let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
					processed_bytes = 0;
					last_notif = Instant::now();
				}
			}
		}
		hasher
			.finalize()
			.iter()
			.map(|b| format!("{:02x}", b))
			.collect::<String>()
	}};
}

macro_rules! blake3_hash_file {
	($f: expr, $buffer: expr, $tx: expr, $alg: ident) => {{
		let mut hasher = $alg::new();
		let mut processed_bytes = 0;
		let mut last_notif = Instant::now();
		let ref_duration = Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
		let mut first_read = true;
		let mut use_rayon = true;
		loop {
			let n = $f.read(&mut $buffer)?;
			if n == 0 {
				if let Some(ref s) = $tx {
					let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
				}
				break;
			}
			if first_read {
				first_read = false;
				use_rayon = n == crate::BUFF_SIZE;
			}
			if use_rayon {
				hasher.update_rayon(&$buffer[..n]);
			} else {
				hasher.update(&$buffer[..n]);
			}
			processed_bytes += n as u64;
			if last_notif.elapsed() >= ref_duration {
				if let Some(ref s) = $tx {
					let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
					processed_bytes = 0;
					last_notif = Instant::now();
				}
			}
		}
		hasher
			.finalize()
			.as_bytes()
			.iter()
			.map(|b| format!("{:02x}", b))
			.collect::<String>()
	}};
}

#[inline]
fn hash_file(
	file: &File,
	hash: HashFunc,
	tx: Option<Sender<InternalHashStatus>>,
) -> io::Result<File> {
	let mut f = fs::File::open(file.get_path())?;
	let mut buffer = [0; crate::BUFF_SIZE];
	let result = match hash {
		HashFunc::Sha256 => alg_hash_file!(f, buffer, tx, Sha256),
		HashFunc::Sha384 => alg_hash_file!(f, buffer, tx, Sha384),
		HashFunc::Sha512 => alg_hash_file!(f, buffer, tx, Sha512),
		HashFunc::Sha3_256 => alg_hash_file!(f, buffer, tx, Sha3_256),
		HashFunc::Sha3_384 => alg_hash_file!(f, buffer, tx, Sha3_384),
		HashFunc::Sha3_512 => alg_hash_file!(f, buffer, tx, Sha3_512),
		HashFunc::Blake2s => alg_hash_file!(f, buffer, tx, Blake2s256),
		HashFunc::Blake2b => alg_hash_file!(f, buffer, tx, Blake2b512),
		HashFunc::Blake3 => blake3_hash_file!(f, buffer, tx, Blake3),
	};
	Ok(File::create_dummy(
		file.get_path(),
		file.get_prefix(),
		file.get_size(),
		&result,
	))
}
