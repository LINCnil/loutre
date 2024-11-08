use crate::events::{send_event_sync, ExternalEvent, ExternalEventSender};
use blake2::{Blake2b512, Blake2s256};
use blake3::Hasher as Blake3;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fmt, io};
use strum::EnumIter;

pub const CHARS_TO_REMOVE: &[char] = &['-', '_', ' '];

macro_rules! alg_hash_file {
	($f: ident, $buffer: ident, $tx: ident, $alg: ident) => {{
		let mut hasher = $alg::new();
		let mut processed_bytes = 0;
		let mut last_notif = Instant::now();
		let ref_duration = Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
		loop {
			let n = $f.read(&mut $buffer)?;
			if n == 0 {
				send_event_sync(&$tx, ExternalEvent::ProgressBarAdd(processed_bytes));
				break;
			}
			hasher.update(&$buffer[..n]);
			processed_bytes += (n as u64);
			if last_notif.elapsed() >= ref_duration {
				if send_event_sync(&$tx, ExternalEvent::ProgressBarAdd(processed_bytes)) {
					processed_bytes = 0;
					last_notif = Instant::now();
				}
			}
		}
		Ok(hasher
			.finalize()
			.iter()
			.map(|b| format!("{:02x}", b))
			.collect::<String>())
	}};
}

macro_rules! blake3_hash_file {
	($f: ident, $buffer: ident, $tx: ident, $alg: ident) => {{
		let mut hasher = $alg::new();
		let mut processed_bytes = 0;
		let mut last_notif = Instant::now();
		let ref_duration = Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
		let mut first_read = true;
		let mut use_rayon = true;
		loop {
			let n = $f.read(&mut $buffer)?;
			if n == 0 {
				send_event_sync(&$tx, ExternalEvent::ProgressBarAdd(processed_bytes));
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
			processed_bytes += (n as u64);
			if last_notif.elapsed() >= ref_duration {
				if send_event_sync(&$tx, ExternalEvent::ProgressBarAdd(processed_bytes)) {
					processed_bytes = 0;
					last_notif = Instant::now();
				}
			}
		}
		Ok(hasher
			.finalize()
			.as_bytes()
			.iter()
			.map(|b| format!("{:02x}", b))
			.collect::<String>())
	}};
}

#[derive(Copy, Clone, Debug, Default, EnumIter, Hash, PartialEq, Eq, Deserialize, Serialize)]
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

impl fmt::Display for HashFunc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Sha256 => "SHA256",
			Self::Sha384 => "SHA384",
			Self::Sha512 => "SHA512",
			Self::Sha3_256 => "SHA3-256",
			Self::Sha3_384 => "SHA3-384",
			Self::Sha3_512 => "SHA3-512",
			Self::Blake2s => "BLAKE2s",
			Self::Blake2b => "BLAKE2b",
			Self::Blake3 => "BLAKE3",
		};
		write!(f, "{s}")
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHashFuncError;

impl std::str::FromStr for HashFunc {
	type Err = ParseHashFuncError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.replace(CHARS_TO_REMOVE, "").to_ascii_lowercase().as_str() {
			"sha256" => Ok(Self::Sha256),
			"sha384" => Ok(Self::Sha384),
			"sha512" => Ok(Self::Sha512),
			"sha3256" => Ok(Self::Sha3_256),
			"sha3384" => Ok(Self::Sha3_384),
			"sha3512" => Ok(Self::Sha3_512),
			"blake2s" => Ok(Self::Blake2s),
			"blake2b" => Ok(Self::Blake2b),
			"blake3" => Ok(Self::Blake3),
			_ => Err(ParseHashFuncError),
		}
	}
}

impl HashFunc {
	pub fn hash_file<P: AsRef<Path>>(
		&self,
		file: P,
		tx: ExternalEventSender,
	) -> io::Result<String> {
		let file = file.as_ref();
		info!("Calculating the {self} hash of file: {}", file.display());
		let mut f = File::open(file)?;
		let mut buffer = [0; crate::BUFF_SIZE];
		match self {
			Self::Sha256 => alg_hash_file!(f, buffer, tx, Sha256),
			Self::Sha384 => alg_hash_file!(f, buffer, tx, Sha384),
			Self::Sha512 => alg_hash_file!(f, buffer, tx, Sha512),
			Self::Sha3_256 => alg_hash_file!(f, buffer, tx, Sha3_256),
			Self::Sha3_384 => alg_hash_file!(f, buffer, tx, Sha3_384),
			Self::Sha3_512 => alg_hash_file!(f, buffer, tx, Sha3_512),
			Self::Blake2s => alg_hash_file!(f, buffer, tx, Blake2s256),
			Self::Blake2b => alg_hash_file!(f, buffer, tx, Blake2b512),
			Self::Blake3 => blake3_hash_file!(f, buffer, tx, Blake3),
		}
	}
}
