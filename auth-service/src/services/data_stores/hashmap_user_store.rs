use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
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

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)
            .cloned()
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => {
                // Note: If password derives Eq, why can't == be used?
                if !user.password.eq(password) {
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
            // Note: To understand this chained calls. Why `to_owned?`
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password123".to_owned()).unwrap(),
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
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password123".to_owned()).unwrap(),
            require_2fa: false,
        };

        store.add_user(user.clone()).await.unwrap();

        let retrieved_used = store
            .get_user(&Email::parse("test@example.com".to_owned()).unwrap())
            .await;

        assert_eq!(retrieved_used, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password123".to_owned()).unwrap(),
            require_2fa: false,
        };

        store.add_user(user.clone()).await.unwrap();

        let wrong_email = Email::parse("nonexistent@example.com".to_owned()).unwrap();
        let wrong_password = Password::parse("any_password".to_owned()).unwrap();

        assert_eq!(
            store.validate_user(&wrong_email, &wrong_password).await,
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store
                .validate_user(
                    &Email::parse("test@example.com".to_owned()).unwrap(),
                    &wrong_password
                )
                .await,
            Err(UserStoreError::InvalidCredentials)
        );

        assert_eq!(
            store
                .validate_user(
                    &Email::parse("test@example.com".to_owned()).unwrap(),
                    &Password::parse("password123".to_owned()).unwrap()
                )
                .await,
            Ok(())
        );
    }
}
