use crate::{
    config::{Config, DatabaseConfig},
    domain::{email, user::Email},
    handlers::{
        health_check::{health_check, HEALTH_CHECK_PATH},
        user::{confirm, login, signup, CONFIRM_PATH, LOGIN_PATH, SIGNUP_PATH, USER_BASE_PATH},
        websockets::ChatManager,
    },
};
use actix::{Actor, Addr};
use actix_web::{
    dev::Server,
    web::{get, post, scope, Data},
    HttpServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.app.host, config.app.port);

        let db_pool = Self::get_connection_pool(&config.database);

        let sender_email = match Email::try_from(config.email_client.sender_email) {
            Ok(email) => email,
            Err(e) => {
                tracing::error!(
                    "failed to parse email_client.sender_email from config: {:?}",
                    e
                );
                panic!(
                    "failed to parse email_client.sender_email from config: {:?}",
                    e
                );
            }
        };

        let email_client = email::Client::new(config.email_client.base_url, sender_email);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let chat_manager = ChatManager::new().start();
        let server = Self::run(listener, db_pool, email_client, chat_manager)?;

        Ok(Self { port, server })
    }

    fn run(
        listener: TcpListener,
        db_pool: PgPool,
        email_client: email::Client,
        chat_manager: Addr<ChatManager>,
    ) -> Result<Server, std::io::Error> {
        let db_pool = Data::new(db_pool);
        let email_client = Data::new(email_client);
        let chat_manager = Data::new(chat_manager);

        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .route(HEALTH_CHECK_PATH, get().to(health_check))
                .service(
                    scope(USER_BASE_PATH)
                        .route(SIGNUP_PATH, post().to(signup))
                        .route(LOGIN_PATH, post().to(login))
                        .route(
                            &format!("{}/{{confirmation_token}}", CONFIRM_PATH),
                            post().to(confirm),
                        ),
                )
                .route("/chat", get().to(ChatManager::chat_route))
                .app_data(db_pool.clone())
                .app_data(email_client.clone())
                .app_data(chat_manager.clone())
        })
        .listen(listener)?
    .run();
        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn get_connection_pool(config: &DatabaseConfig) -> PgPool {
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(config.with_db())
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
