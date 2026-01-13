use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;

pub struct MmapStore {
    pub blob: Arc<Mmap>,
}

impl MmapStore {
    pub fn open_file(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
        let mmap = unsafe { Mmap::map(&file).map_err(|e| e.to_string())? };

        Ok(Self {
            blob: Arc::new(mmap),
        })
    }

    /// Returns the raw window into the file
    pub fn as_bytes(&self) -> &[u8] {
        &self.blob
    }

    /// Calculates row count based on the "Width" of the data type
    pub fn count_rows(&self, byte_width: usize) -> usize {
        self.blob.len() / byte_width
    }
}