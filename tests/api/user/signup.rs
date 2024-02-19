use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use lightr_server::{
    handlers::user::{SIGNUP_PATH, USER_BASE_PATH},
    storage::{get_user_by_email, USERS_TABLE_NAME},
};
use secrecy::Secret;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[actix::test]
async fn test_signup_success() {
    let app = TestApp::spawn().await;

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let body =
        "name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness";
    let response = app
        .client
        .request(
            Path::POST(format!("{}{}", USER_BASE_PATH, SIGNUP_PATH)),
            &[Header::ContentType(ContentType::FormURLEncoded)],
            Some(body),
        )
        .await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return 200 when signing up with valid login details",
    );

    match get_user_by_email(&app.database.db_pool, "alex.pitsikoulis@youwish.com").await {
        Ok(user) => {
            assert_eq!(
                "alex.pitsikoulis",
                user.name(),
                "Inserted name does not match the one provided in the request",
            );
            assert_eq!(
                "alex.pitsikoulis@youwish.com",
                user.email().as_ref(),
                "Inserted email does not match the one provided in the request",
            );
            assert!(
                user.password()
                    .compare(&Secret::new("N0neofyourbus!ness".into())),
                "Inserted password does not match the one provided in the request",
            );
        }
        Err(e) => {
            panic!("DB query failed: {}", e);
        }
    };
}

#[actix::test]
async fn test_signup_failed_400() {
    let mut app = TestApp::spawn().await;
    let test_cases = vec![
        ("name=alex.pitsikoulis0&password=N0neofyourbus!ness", "missing the email"),
        ("email=alex.pitsikoulis%40youwish.com&password=N0neofyourbus!ness", "the name"),
        ("email=alex.pitsikoulis%40youwish.com&name=alex.pitsikoulis", "missing the password"),
        ("name=alex.pitsikoulis", "missing the email and password"),
        ("email=alex.pitsikoulis%40youwish.com", "missing the name and password"),
        ("password=N0neofyourbus!ness", "missing the name and email"),
        ("", "missing the name, email, and password"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=c0!", "password too short"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!c0!", "password too long"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=passw0rd!", "password missing uppercase letter"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=Password!", "password missing number"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=Passw0rd", "password missing special character"),
        ("name=alex.pitsikoulis&email=alex.pitsikoulis%40youwish.com&password=PASSW0RD!", "password missing lowercase letter"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .client
            .request(
                Path::POST(format!("{}{}", USER_BASE_PATH, SIGNUP_PATH)),
                &[Header::ContentType(ContentType::FormURLEncoded)],
                Some(invalid_body),
            )
            .await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
        app.database.clear(USERS_TABLE_NAME).await;
    }
}
