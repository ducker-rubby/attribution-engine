pub trait Event {
    fn get_metadata(&self) -> Vec<(&str, &str)>;
}
