#[derive(Debug)]
pub struct StorageObject {
    key: String,
    value: String,
}

impl StorageObject {
    pub fn new_storage_obj(key: String, value: String) -> StorageObject {
        StorageObject { key, value }
    }

    pub fn write(&self) {
        let key = &self.key;
        let value = &self.value;
    }
}