use std::{ops::Index, slice::SliceIndex};

#[derive(Debug, Default, Clone)]
pub struct FileProperties {
    pub file_name: String,
    pub file_size: f32,
    pub is_folder: i8,
    pub path: String,
}

#[derive(Debug, Default, Clone)]
pub struct File {
    pub is_error: bool,
    pub properties: FileProperties,
    pub content: Vec<String>,
    pub children: Vec<File>,
}

pub const BLOCK_SIZE: usize = 8;
