use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    Int32,
    Int64,
    Float64,
    String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: DataType,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}

impl TableSchema {
    /// loads the schema from JSON file
    pub fn from_file(path: &str) -> Result<Self, String> {
        let json_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let schema: TableSchema = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
        Ok(schema)
    }

    /// finds a specific column's metadata by name
    pub fn get_column(&self, name: &str) -> Option<&ColumnSchema> {
        self.columns.iter().find(|c| c.name == name)
    }
}