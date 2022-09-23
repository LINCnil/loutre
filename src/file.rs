use crate::path_cmp::path_cmp_name;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Clone)]
pub struct File {
    path: PathBuf,
    prefix: PathBuf,
    size: u64,
    hash: Option<String>,
}

impl File {
    pub fn new(path: &Path, prefix: &Path) -> io::Result<Self> {
        let metadata = path.metadata()?;
        Ok(File {
            path: path.to_owned(),
            prefix: prefix.to_owned(),
            size: metadata.len(),
            hash: None,
        })
    }

    pub fn create_dummy(path: &Path, prefix: &Path, size: u64, hash: &str) -> Self {
        File {
            path: path.to_owned(),
            prefix: prefix.to_owned(),
            size,
            hash: Some(hash.into()),
        }
    }

    pub fn get_content_file_line(&self) -> Vec<u8> {
        let hash = match &self.hash {
            Some(s) => s.to_owned(),
            None => String::new(),
        };
        format!(
            "{}{}\t{}\t{}\r\n",
            crate::CONTENT_FILE_PATH_PREFIX,
            self.get_file_name().display(),
            self.size,
            &hash
        )
        .into()
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn get_prefix(&self) -> &Path {
        self.prefix.as_path()
    }

    pub fn get_file_name(&self) -> PathBuf {
        match self.path.strip_prefix(&self.prefix) {
            Ok(p) => p.to_owned(),
            Err(_) => self.path.clone(),
        }
    }

    pub fn get_hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn display_file_name(&self) -> String {
        self.get_file_name().display().to_string()
    }

    pub fn set_readonly(&self) -> io::Result<()> {
        let metadata = self.path.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&self.path, permissions)?;
        Ok(())
    }

    pub fn cmp_name(a: &File, b: &File) -> Ordering {
        path_cmp_name(&a.path, &b.path)
    }
}
