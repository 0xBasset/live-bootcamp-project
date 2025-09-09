use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!({
        "password": "password124",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        )
    }
}

#[tokio::test]

async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let inputs = vec![
        (random_email.clone(), "short", true),
        ("invalid-email".to_string(), "password124", false),
        ("".to_string(), "password124", true),
    ];

    let test_cases: Vec<_> = inputs
        .into_iter()
        .map(|(email, password, requires2fa)| {
            serde_json::json!({
                "email": email,
                "password": password,
                "requires2FA": requires2fa
            })
        })
        .collect();

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        )
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [serde_json::json!({
        "email": random_email,
        "password": "password124",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}",
            test_case
        );

        let failed_response = app.post_signup(test_case).await;

        assert_eq!(
            failed_response.status().as_u16(),
            409,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [serde_json::json!({
        "email": random_email,
        "password": "password124",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}",
            test_case
        )
    }
}
