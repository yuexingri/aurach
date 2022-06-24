pub trait Reader {
    fn read_by_key(&self, key: String);
}