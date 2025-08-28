// Note: Both email and password are almost the same except the validate logic. Is there a way to reduce this duplications?

// TODO:
/**
 * [dependencies]
#...
validator = "0.16.1"

[dev-dependencies]
#...
fake = "=2.3.0"
quickcheck = "0.9.2"
quickcheck_macros = "0.9.1"
NOTE: Make sure to use the exact versions above.

validator - provides common validation functions for emails, URLs, and more!
fake - an easy to use library for generating fake data like name, number, address, lorem, dates, etc. This is useful for unit tests!
quickcheck - provides a way to do property-based testing using randomly generated input. Property-based testing is a testing approach where you define properties (invariants, rules, or behaviors) that should always hold true for your code. The testing framework then automatically generates a wide range of inputs and checks if your code maintains those properties for all generated cases. This is useful for unit tests!
quickcheck_macros - provides a convenient quickcheck! macro.
 */

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
