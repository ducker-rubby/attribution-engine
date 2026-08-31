use crate::models::Event;

#[derive(Debug)]
pub struct ClickEvent<'a> {
    id: &'a str,
    metadata: Vec<(&'a str, &'a str)>,
}

impl<'a> ClickEvent<'a> {
    pub fn build(id: &'a str) -> Self {
        let metadata = vec![("link_id", id)];
        Self { id, metadata }
    }
}

impl Event for ClickEvent<'_> {
    fn get_type(&self) -> &str {
        unimplemented!()
    }

    fn get_id(&self) -> &str {
        self.id
    }

    fn get_metadata(&self) -> &[(&str, &str)] {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_click_event() {
        let click_event = ClickEvent::build("abcd");

        assert_eq!(click_event.get_id(), "abcd");
        assert_eq!(click_event.get_metadata(), &[("link_id", "abcd")]);
    }
}
