#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(e: String) -> Result<Self, String> {
        if e.len() < 8 {
            // Why a return statement is necessary here? Where in some places the lack of ";" implies a return
            return Err("Password too short".to_string());
        }

        Ok(Password(e))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
