use uuid::Uuid;

use crate::models::Event;

#[derive(Debug)]
pub struct ClickEvent<'a> {
    link_id: &'a str,
    click_ref_str: String,
}

impl<'a> ClickEvent<'a> {
    pub fn build(id: &'a str) -> Self {
        let uuid = Uuid::now_v7();
        Self {
            link_id: id,
            click_ref_str: uuid.to_string(),
        }
    }
}

impl Event for ClickEvent<'_> {
    fn get_metadata(&self) -> Vec<(&str, &str)> {
        vec![
            ("link_id", self.link_id),
            ("click_ref", &self.click_ref_str),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_click_event() {
        let click_event = ClickEvent::build("abcd");
        let metadata = click_event.get_metadata();

        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0], ("link_id", "abcd"));
        assert_eq!(metadata[1].0, "click_ref");

        let click_ref_str = metadata[1].1;
        let parsed_uuid = Uuid::parse_str(click_ref_str).expect("Click ref should be a valid UUID");

        assert_eq!(
            parsed_uuid.get_version_num(),
            7,
            "click_ref must be UUID v7"
        );
    }
}
