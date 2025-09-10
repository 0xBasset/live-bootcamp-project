use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashSetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    // Note: Is the Result Necessary if insert and contains can't fail?
    async fn store(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }
    async fn exists(&self, token: String) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(&token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_token() {
        let mut store = HashSetBannedTokenStore::default();
        let token = "banned_token_123".to_owned();

        // Test storing a token
        assert_eq!(store.store(token.clone()).await, Ok(()));

        //Test storing the same token again (should still succeed)
        assert_eq!(store.store(token).await, Ok(()));
    }

    #[tokio::test]
    async fn test_exists_token() {
        let mut store = HashSetBannedTokenStore::default();
        let token = "banned_token_456".to_owned();

        // Test that token doesn't exist initially
        assert_eq!(store.exists(token.clone()).await, Ok(false));

        // Store the token
        store.store(token.clone()).await.unwrap();

        // Test that token now exists
        assert_eq!(store.exists(token).await, Ok(true));

        // Test that a different token doesn't exist
        assert_eq!(store.exists("different_token".to_owned()).await, Ok(false));
    }

    #[tokio::test]
    async fn test_store_multiple_tokens() {
        let mut store = HashSetBannedTokenStore::default();
        let token1 = "token_one".to_owned();
        let token2 = "token_two".to_owned();
        let token3 = "token_three".to_owned();

        // Store multiple tokens
        store.store(token1.clone()).await.unwrap();
        store.store(token2.clone()).await.unwrap();
        store.store(token3.clone()).await.unwrap();

        // Verify all tokens exist
        assert_eq!(store.exists(token1).await, Ok(true));
        assert_eq!(store.exists(token2).await, Ok(true));
        assert_eq!(store.exists(token3).await, Ok(true));

        // Verify non-existent token
        assert_eq!(
            store.exists("non_existent_token".to_owned()).await,
            Ok(false)
        );
    }
}
