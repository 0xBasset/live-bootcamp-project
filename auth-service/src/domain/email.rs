// Note: Both email and password are almost the same except the validate logic. Is there a way to reduce this duplications?

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(e: String) -> Result<Self, String> {
        if !e.contains("@") {
            return Err("not valid email - missing @".to_string());
        }
        Ok(Email(e))
    }
}

// Note: Don't fully understand this
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
