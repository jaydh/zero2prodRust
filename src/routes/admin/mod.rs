mod dashboard;
mod logout;
mod password;

pub use dashboard::{admin_dashboard, get_username};
pub use logout::log_out;
pub use password::{change_password, change_password_form};
