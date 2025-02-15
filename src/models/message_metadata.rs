// Message metadata struct
#[derive(Debug, Clone, Default)]
pub struct MessageMetadata<'a> {
    from: &'a str,
    to: Vec<&'a str>,
    subject: Option<&'a str>,
}

impl<'a> MessageMetadata<'a> {
    pub fn new(from: &'a str, to: Vec<&'a str>, subject: Option<&'a str>) -> Self {
        Self { from, to, subject }
    }
    pub fn from(&self) -> &str {
        self.from
    }

    #[allow(dead_code)]
    pub fn to(&self) -> &[&str] {
        &self.to
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject
    }
}
