mod dashboard;
mod logout;
mod newsletters;
mod password;

pub use dashboard::{admin_dashboard, get_username};
pub use logout::log_out;
pub use newsletters::{publish_newsletter, publish_newsletter_form};
pub use password::{change_password, change_password_form};
