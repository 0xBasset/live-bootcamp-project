use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)
            .cloned()
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => {
                if user.password != password {
                    return Err(UserStoreError::InvalidCredentials);
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::User;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::new();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        assert_eq!(store.add_user(user.clone()), Ok(()));
        assert_eq!(store.add_user(user), Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::new();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        store.add_user(user.clone()).unwrap();

        let retrieved_used = store.get_user("test@example.com").unwrap();
        assert_eq!(retrieved_used.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::new();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        store.add_user(user.clone()).unwrap();

        assert_eq!(
            store.validate_user("nonexistent@example.com", "any_password"),
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store.validate_user("test@example.com", "wrong_password"),
            Err(UserStoreError::InvalidCredentials)
        );

        assert_eq!(
            store.validate_user("test@example.com", "password123"),
            Ok(())
        );
    }
}
