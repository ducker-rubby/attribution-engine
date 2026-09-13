//TODO: Should this be in it's own file?
pub trait Event {
    fn get_metadata(&self) -> Vec<(&str, &str)>;
}
