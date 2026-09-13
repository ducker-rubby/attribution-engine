//FIX: This struct is the same as models::link::Link

//TODO: This file needs a get link struct
pub struct CreateLink {
    pub name: String,
    pub redirect_url: String,
    pub group_id: Option<i32>,
}

impl CreateLink {
    pub fn build(name: impl Into<String>, redirect_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            redirect_url: redirect_url.into(),
            group_id: None,
        }
    }

    pub fn with_group(mut self, group_id: i32) -> Self {
        self.group_id = Some(group_id);
        self
    }
}
