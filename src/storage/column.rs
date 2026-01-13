use crate::catalog::schema::DataType;
use crate::storage::mmap_store::MmapStore;
use std::sync::Arc;

pub enum ColumnVault {
    WholeNumbers(Arc<MmapStore>),    
    MoneyAndScores(Arc<MmapStore>), 
    TextLabels(Arc<MmapStore>),     
}

pub struct ColumnManager {
    pub name: String,
    pub vault: ColumnVault,
}

impl ColumnManager {
    pub fn load_from_disk(name: &str, dtype: &DataType, filename: &str) -> Result<Self, String> {
        let store = Arc::new(MmapStore::open_file(filename)?);

        let vault = match dtype {
            DataType::Int32 => ColumnVault::WholeNumbers(store),
            DataType::Float64 => ColumnVault::MoneyAndScores(store),
            DataType::String => ColumnVault::TextLabels(store),
            _ => return Err(format!("Type {:?} not supported in ColumnVault", dtype)),
        };

        Ok(Self {
            name: name.to_string(),
            vault,
        })
    }

    pub fn dispatch_to_math<F>(&self, mut logic: F) 
    where 
        F: FnMut(&ColumnVault) 
    {
        logic(&self.vault);
    }
}