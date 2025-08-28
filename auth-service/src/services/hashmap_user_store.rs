use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)
            .cloned()
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        assert_eq!(store.add_user(user.clone()).await, Ok(()));
        assert_eq!(
            store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        store.add_user(user.clone()).await.unwrap();

        let retrieved_used = store.get_user("test@example.com").await.unwrap();
        assert_eq!(retrieved_used.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            require_2fa: false,
        };

        store.add_user(user.clone()).await.unwrap();

        assert_eq!(
            store
                .validate_user("nonexistent@example.com", "any_password")
                .await,
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store
                .validate_user("test@example.com", "wrong_password")
                .await,
            Err(UserStoreError::InvalidCredentials)
        );

        assert_eq!(
            store.validate_user("test@example.com", "password123").await,
            Ok(())
        );
    }
}
