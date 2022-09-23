use crate::file::File;
use crate::file_list::FileList;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fs, thread};

enum InternalHashStatus {
    BytesConsumed(u64),
    NewFile(File),
    Error(String),
}

pub enum HashStatus {
    NewFile(File),
    Error(String),
    Finished,
    None,
}

struct FileJob {
    file: File,
    taken: bool,
}

impl FileJob {
    fn cmp_size(a: &FileJob, b: &FileJob) -> Ordering {
        b.file.get_size().cmp(&a.file.get_size())
    }
}

impl From<&File> for FileJob {
    fn from(f: &File) -> Self {
        FileJob {
            file: f.to_owned(),
            taken: false,
        }
    }
}

pub struct FileHasher {
    rx: Receiver<InternalHashStatus>,
    processed_bytes: u64,
    total_bytes: u64,
}

impl FileHasher {
    pub fn new(file_list: &FileList) -> Self {
        // Define some king of metadata
        let nb_threads = match thread::available_parallelism() {
            Ok(nb) => nb.get(),
            Err(_) => 1,
        };
        let total_bytes = file_list.get_total_size();
        let (base_tx, rx) = channel();

        // Generate the shared job list
        let mut job_lst: Vec<FileJob> = file_list.iter_files().map(FileJob::from).collect();
        job_lst.sort_by(FileJob::cmp_size);
        let shared_lst = Arc::new(Mutex::new(job_lst));

        // Spawn hashing threads on each list
        for _ in 0..nb_threads {
            let tx = base_tx.clone();
            let jobs = shared_lst.clone();
            thread::spawn(move || loop {
                let mut mut_lst = jobs.lock().unwrap();
                let file = match (*mut_lst).iter_mut().find(|j| !j.taken) {
                    Some(mut job) => {
                        job.taken = true;
                        job.file.clone()
                    }
                    None => {
                        break;
                    }
                };
                std::mem::drop(mut_lst);
                let _ = match hash_file(&file, Some(tx.clone())) {
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
        }
    }

    pub fn update_status(&mut self) -> HashStatus {
        match self.rx.try_recv() {
            Ok(rsp) => match rsp {
                InternalHashStatus::BytesConsumed(nb) => {
                    self.processed_bytes += nb;
                    HashStatus::None
                }
                InternalHashStatus::NewFile(f) => HashStatus::NewFile(f),
                InternalHashStatus::Error(e) => HashStatus::Error(e),
            },
            Err(e) => match e {
                TryRecvError::Empty => HashStatus::None,
                TryRecvError::Disconnected => HashStatus::Finished,
            },
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

pub fn hash_single_file(file: &File) -> io::Result<File> {
    hash_file(file, None)
}

fn hash_file(file: &File, tx: Option<Sender<InternalHashStatus>>) -> io::Result<File> {
    let mut f = fs::File::open(file.get_path())?;
    let mut buffer = [0; crate::BUFF_SIZE];
    let mut hasher = Sha256::new();
    let mut processed_bytes = 0;
    let mut last_notif = Instant::now();
    let ref_duration = Duration::from_millis(crate::BUFF_NOTIF_THRESHOLD);
    loop {
        let n = f.read(&mut buffer)?;
        if n == 0 {
            if let Some(ref s) = tx {
                let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
            }
            break;
        }
        hasher.update(&buffer[..n]);
        processed_bytes += n as u64;
        if last_notif.elapsed() >= ref_duration {
            if let Some(ref s) = tx {
                let _ = s.send(InternalHashStatus::BytesConsumed(processed_bytes));
                processed_bytes = 0;
                last_notif = Instant::now();
            }
        }
    }
    let result = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    Ok(File::create_dummy(
        file.get_path(),
        file.get_prefix(),
        file.get_size(),
        &result,
    ))
}
