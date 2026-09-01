use crate::models::Event;

#[derive(Debug)]
pub struct ClickEvent<'a> {
    id: &'a str,
    metadata: Vec<(&'a str, &'a str)>,
    click_ref: &'a str,
}

impl<'a> ClickEvent<'a> {
    pub fn build(id: &'a str) -> Self {
        let click_ref = ClickEvent::new_ref_id();
        let metadata = vec![("link_id", id), ("click_ref", click_ref)];

        Self {
            id,
            metadata,
            click_ref,
        }
    }

    fn new_ref_id() -> &'a str {
        unimplemented!()
    }
}

impl Event for ClickEvent<'_> {
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

        assert_eq!(
            click_event.get_metadata(),
            &[("link_id", "abcd"), ("click_ref", "")]
        );
    }
}
