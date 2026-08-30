pub trait Event {
    fn get_id(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_metadata(&self) -> &[(&str, &str)];
}
