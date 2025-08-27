#[derive(Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    pub require_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email: email,
            password: password,
            require_2fa: requires_2fa,
        }
    }
}
