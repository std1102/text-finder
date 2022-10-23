pub struct FileProperties {
    pub file_name: String,
    pub file_size: f32,
    pub is_folder: bool,
    pub path: String
}

pub struct File {
    pub properties: FileProperties,
    pub content: Vec<Vec<String>>,
}
