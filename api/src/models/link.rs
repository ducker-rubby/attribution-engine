//TODO: move redirect struct outside to own file
#[derive(Debug)]
pub struct Link {
    pub id: String,
    pub redirect_url: String,
}

impl Link {
    pub fn build(link_id: impl Into<String>, redirect_url: impl Into<String>) -> Self {
        Link {
            id: link_id.into(),
            redirect_url: redirect_url.into(),
        }
    }
}
