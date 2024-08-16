use validator::validate_email;
use crate::types::types::EmailOrUsername;

pub fn determine_login_contact_type(input: &str) -> EmailOrUsername {
    if validate_email(input) {
        EmailOrUsername::Email
    } else {
        EmailOrUsername::Username
    }
}
