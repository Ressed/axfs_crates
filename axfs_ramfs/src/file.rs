use alloc::vec::Vec;
use axfs_vfs::{impl_vfs_non_dir_default, VfsNodeAttr, VfsNodeOps, VfsResult};
use spin::RwLock;

/// The file node in the RAM filesystem.
///
/// It implements [`axfs_vfs::VfsNodeOps`].
pub struct FileNode {
    content: RwLock<Vec<u8>>,
    metadata: RwLock<Metadata>,
}

struct Metadata {
    atime: usize,
    mtime: usize,
}

impl FileNode {
    pub(super) const fn new() -> Self {
        Self {
            content: RwLock::new(Vec::new()),
            metadata: RwLock::new(Metadata { atime: 0, mtime: 0 }),
        }
    }
}

impl VfsNodeOps for FileNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        let mut attr = VfsNodeAttr::new_file(self.content.read().len() as _, 0);
        let metadata = self.metadata.read();
        attr.set_atime(metadata.atime);
        attr.set_mtime(metadata.mtime);
        Ok(attr)
    }

    fn truncate(&self, size: u64) -> VfsResult {
        let mut content = self.content.write();
        if size < content.len() as u64 {
            content.truncate(size as _);
        } else {
            content.resize(size as _, 0);
        }
        Ok(())
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let content = self.content.read();
        let start = content.len().min(offset as usize);
        let end = content.len().min(offset as usize + buf.len());
        let src = &content[start..end];
        buf[..src.len()].copy_from_slice(src);
        Ok(src.len())
    }

    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let offset = offset as usize;
        let mut content = self.content.write();
        if offset + buf.len() > content.len() {
            content.resize(offset + buf.len(), 0);
        }
        let dst = &mut content[offset..offset + buf.len()];
        dst.copy_from_slice(&buf[..dst.len()]);
        Ok(buf.len())
    }

    fn set_atime(&self, atime: usize) -> VfsResult {
        let mut metadata = self.metadata.write();
        metadata.atime = atime;
        Ok(())
    }

    fn set_mtime(&self, mtime: usize) -> VfsResult {
        let mut metadata = self.metadata.write();
        metadata.mtime = mtime;
        Ok(())
    }

    impl_vfs_non_dir_default! {}
}
