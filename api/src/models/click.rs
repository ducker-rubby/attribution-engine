#[derive(Debug)]
pub struct Click<'a> {
    id: &'a str,
}

impl<'a> Click<'a> {
    pub fn build(id: &'a str) -> Self {
        Self { id }
    }
}
