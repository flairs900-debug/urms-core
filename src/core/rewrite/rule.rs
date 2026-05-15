#[derive(Clone)]
pub struct RewriteRule {
    pub from: String,
    pub to: String,
}

impl RewriteRule {
    pub fn new(
        from: &str,
        to: &str,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}