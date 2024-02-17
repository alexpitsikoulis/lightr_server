#[cfg(test)]
mod tests {
    use crate::{
        domain::user::credentials::{Email, Password},
        utils::test::PASSWORD_GENERATOR,
    };
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use secrecy::Secret;

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            ValidEmailFixture(email)
        }
    }
    #[test]
    fn invalid_email_rejected() {
        let emails = &[
            "alex",
            "alex.pitsikoulis@test",
            "alex@test.",
            "alex@test.qwertyuiop",
            "_alex@test.com",
            "alex_@test.com",
            "alex@test.com_",
            "alex@_test.com",
        ];
        for email in emails {
            assert_err!(Email::try_from(email.to_string()));
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_parsed_successfully(email: ValidEmailFixture) -> bool {
        Email::try_from(email.0).is_ok()
    }

    #[derive(Clone, Debug)]
    struct ValidPasswordFixture(pub Secret<String>);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let password = PASSWORD_GENERATOR.generate(g);
            ValidPasswordFixture(Secret::new(password))
        }
    }

    #[test]
    fn fails_when_less_than_8_grapheme() {
        let password = Secret::new("P@ssw0r".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_more_than_64_grapheme() {
        let filler = "A".repeat(60);
        let password = Secret::new(format!("P@ss1{}", filler).to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_uppercase() {
        let password = Secret::new("n0neofyourbus!ness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_lowercase() {
        let password = Secret::new("N0NEOFYOURBUS!NESS".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_number() {
        let password = Secret::new("Noneofyourbus!ness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_special_char() {
        let password = Secret::new("N0neofyourbusiness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_password_parses_successfully(password: ValidPasswordFixture) -> bool {
        Password::try_from(password.0).is_ok()
    }
}