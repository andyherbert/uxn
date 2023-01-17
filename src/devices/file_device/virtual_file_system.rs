use super::{file_entry, ReadType};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    io::{self, ErrorKind},
    rc::Rc,
};

#[derive(Clone, Default)]
pub struct VirtualFileSystem {
    files: Rc<RefCell<HashMap<String, Vec<u8>>>>,
}

impl VirtualFileSystem {
    pub fn stat(&self, path: &str) -> Option<Vec<u8>> {
        let len = self.files.borrow().get(path)?.len();
        let bytes = file_entry(path, len);
        Some(bytes)
    }

    pub fn delete(&mut self, path: &str) {
        self.files.borrow_mut().remove(path);
    }

    pub fn create(&mut self, path: &str) {
        self.files.borrow_mut().insert(path.to_string(), vec![]);
    }

    pub fn write(&mut self, path: &str, buf: &[u8]) -> Result<usize, Box<dyn Error>> {
        match self.files.borrow_mut().get_mut(path) {
            Some(bytes) => {
                bytes.extend(buf.to_vec());
                Ok(buf.len())
            }
            None => Err(Box::new(io::Error::new(ErrorKind::NotFound, "Not found"))),
        }
    }

    pub fn read(&self, path: &str) -> Result<ReadType, Box<dyn Error>> {
        if path == "./" {
            let bytes = self
                .files
                .borrow()
                .iter()
                .map(|(name, bytes)| file_entry(name, bytes.len()))
                .collect();
            Ok(ReadType::Stats(bytes))
        } else {
            match self.files.borrow().get(path) {
                Some(bytes) => Ok(ReadType::File(bytes.clone())),
                None => Err(Box::new(io::Error::new(ErrorKind::NotFound, "Not found"))),
            }
        }
    }
}
