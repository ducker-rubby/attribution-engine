pub trait Event {
    fn get_metadata(&self) -> &[(&str, &str)];
}
