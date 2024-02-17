use chrono::Utc;
use lightr_server::{
    domain::user::{Email, Password, User},
    storage::{get_user_by_id, upsert_user},
};
use secrecy::Secret;
use uuid::Uuid;

pub const TEST_USER_PASSWORD: &str = "Testp@ssw0rd!";

use super::TestDB;

impl TestDB {
    pub async fn insert_user(&mut self, email: &str, name: &str, email_confirmed: bool) -> User {
        let now = Utc::now();
        let user = User::new(
            Uuid::new_v4(),
            Email::try_from(email).expect(&format!("Email '{}' is invalid", email)),
            Password::try_from(Secret::new(TEST_USER_PASSWORD.into())).unwrap(),
            name.into(),
            None,
            0,
            email_confirmed,
            now,
            now,
            None,
        );

        match upsert_user(&self.db_pool, &user).await {
            Ok(_) => user,
            Err(e) => panic!("Failed to insert user: {:?}", e),
        }
    }

    pub async fn get_user_by_id(&mut self, id: Uuid) -> User {
        match get_user_by_id(&self.db_pool, id).await {
            Ok(user) => user,
            Err(e) => panic!("failed to get user by id: {:?}", e),
        }
    }
}
