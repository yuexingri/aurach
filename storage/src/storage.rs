#[derive(Debug)]
pub struct StorageObject {
    name: String,
}

impl StorageObject {

    pub fn new_storage_obj(name: String) -> StorageObject {
        StorageObject {
            name: name,
        }
    }

    pub fn say_hello(&self) {
        println!("say hello from storage {}", self.name);
    }

}