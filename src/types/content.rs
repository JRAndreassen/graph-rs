#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content {
    content: String,
}

impl Content {
    pub fn new(content: &str) -> Content {
        Content {
            content: content.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.content.as_str()
    }
}

impl From<String> for Content {
    fn from(content: String) -> Self {
        Content { content }
    }
}

impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        self.content.as_str()
    }
}

impl ToString for Content {
    fn to_string(&self) -> String {
        self.content.to_string()
    }
}
