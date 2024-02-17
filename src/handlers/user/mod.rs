mod confirm;
mod login;
mod signup;

pub const USER_BASE_PATH: &str = "/users";

pub use confirm::{confirm, CONFIRM_PATH};
pub use login::{login, LoginForm, LOGIN_PATH};
pub use signup::{signup, UserSignupFormData, SIGNUP_PATH};
