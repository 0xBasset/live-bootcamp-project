use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::Email;
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::utils::constants::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [serde_json::json!({
        "password": "",
    })];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;

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
        (random_email.clone(), "short"),
        ("invalid-email".to_string(), "password124"),
        ("".to_string(), "password124"),
    ];

    let test_cases: Vec<_> = inputs
        .into_iter()
        .map(|(email, password)| {
            serde_json::json!({
                "email": email,
                "password": password,
            })
        })
        .collect();

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        )
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_password() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let user_credentials = serde_json::json!({
        "email": random_email,
        "password": "password124",
        "requires2FA": false
    });

    let response = app.post_signup(&user_credentials).await;

    assert_eq!(response.status().as_u16(), 201, "Failed to create user");

    let incorrect_credentials = serde_json::json!({
        "email": random_email,
        "password": "wrongPassword124"
    });

    let failed_response = app.post_login(&incorrect_credentials).await;

    assert_eq!(
        failed_response.status().as_u16(),
        401,
        "Failed test with incorrect credential"
    );
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let attempt_id = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email).unwrap())
        .await
        .expect("Failed to Get 2fa");

    assert_eq!(attempt_id.0.as_ref(), json_body.login_attempt_id)
}
