use super::{PhysicalFileSystem, ReadType, VirtualFileSystem};
use std::error::Error;

#[derive(Clone)]
pub enum FileInterface {
    FileSystem(PhysicalFileSystem),
    VirtualFileSystem(VirtualFileSystem),
}

impl Default for FileInterface {
    fn default() -> FileInterface {
        FileInterface::FileSystem(PhysicalFileSystem::default())
    }
}

impl FileInterface {
    pub fn stat(&self, path: &str) -> Option<Vec<u8>> {
        match self {
            FileInterface::FileSystem(fs) => fs.stat(path),
            FileInterface::VirtualFileSystem(fs) => fs.stat(path),
        }
    }

    pub fn delete(&mut self, path: &str) {
        match self {
            FileInterface::FileSystem(fs) => fs.delete(path),
            FileInterface::VirtualFileSystem(fs) => fs.delete(path),
        }
    }

    pub fn create(&mut self, path: &str) {
        match self {
            FileInterface::FileSystem(fs) => fs.create(path),
            FileInterface::VirtualFileSystem(fs) => fs.create(path),
        }
    }

    pub fn write(&mut self, path: &str, buf: &[u8]) -> Result<usize, Box<dyn Error>> {
        match self {
            FileInterface::FileSystem(fs) => fs.write(path, buf),
            FileInterface::VirtualFileSystem(fs) => fs.write(path, buf),
        }
    }

    pub fn read(&self, path: &str) -> Result<ReadType, Box<dyn Error>> {
        match self {
            FileInterface::FileSystem(fs) => fs.read(path),
            FileInterface::VirtualFileSystem(fs) => fs.read(path),
        }
    }
}
