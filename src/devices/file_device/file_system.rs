use super::{file_entry, ReadType};
use std::{
    error::Error,
    ffi::OsStr,
    fs::{read_dir, remove_file, File, Metadata, OpenOptions},
    io::{self, ErrorKind, Read, Write},
    path::{Component, Path, PathBuf},
};

fn safety_check(cwd: &Path, path: &str, safety: bool) -> Result<PathBuf, io::Error> {
    let path = Path::new(path);
    if safety
        && path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "Permisssion denied",
        ))
    } else {
        Ok(cwd.join(path))
    }
}

fn stat_entry(metadata: Metadata, name: impl AsRef<OsStr>) -> Option<Vec<u8>> {
    let name = name.as_ref().to_string_lossy();
    if metadata.is_dir() {
        let string = format!("---- {name}\n");
        Some(string.into_bytes())
    } else if metadata.is_file() {
        let bytes = file_entry(name, metadata.len() as usize);
        Some(bytes)
    } else {
        None
    }
}

#[derive(Clone)]
pub struct PhysicalFileSystem {
    cwd: PathBuf,
    safety: bool,
}

impl Default for PhysicalFileSystem {
    fn default() -> Self {
        Self {
            cwd: Default::default(),
            safety: true,
        }
    }
}

impl PhysicalFileSystem {
    pub fn new(path: impl AsRef<Path>, safety: bool) -> PhysicalFileSystem {
        PhysicalFileSystem {
            cwd: PathBuf::from(path.as_ref()),
            safety,
        }
    }

    pub fn stat(&self, path: &str) -> Option<Vec<u8>> {
        let path = safety_check(&self.cwd, path, self.safety).ok()?;
        let metadata = path.metadata().ok()?;
        let name = path.file_name()?;
        stat_entry(metadata, name)
    }

    pub fn delete(&mut self, path: &str) {
        if let Ok(path) = safety_check(&self.cwd, path, self.safety) {
            remove_file(path).ok();
        }
    }

    pub fn create(&mut self, path: &str) {
        if let Ok(path) = safety_check(&self.cwd, path, self.safety) {
            File::create(path).ok();
        }
    }

    pub fn write(&mut self, path: &str, buf: &[u8]) -> Result<usize, Box<dyn Error>> {
        let path = safety_check(&self.cwd, path, self.safety)?;
        let mut file = OpenOptions::new().write(true).append(true).open(path)?;
        file.write_all(buf)?;
        Ok(buf.len())
    }

    pub fn read(&self, path: &str) -> Result<ReadType, Box<dyn Error>> {
        let path = safety_check(&self.cwd, path, self.safety)?;
        let mut file = File::open(&path)?;
        if path.is_file() {
            let mut bytes = vec![];
            file.read_to_end(&mut bytes)?;
            Ok(ReadType::File(bytes))
        } else if path.is_dir() {
            let read_dir = read_dir(path)?;
            let bytes = read_dir
                .into_iter()
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;
                    let name = entry.file_name();
                    stat_entry(metadata, name)
                })
                .collect();
            Ok(ReadType::Stats(bytes))
        } else {
            Err(Box::new(io::Error::new(ErrorKind::NotFound, "Not found")))
        }
    }
}
