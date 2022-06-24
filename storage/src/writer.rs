pub trait Writer {
    fn write(&self, key: String, value: String);
}