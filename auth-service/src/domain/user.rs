use super::Email;
use super::Password;

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub require_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email: email,
            password: password,
            require_2fa: requires_2fa,
        }
    }
}
