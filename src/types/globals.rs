pub struct CredentialsData {
    pub username: String,
    pub hashed_password: String,
    pub email: String
}

pub enum EmailOrUsername {
    Email,
    Username
}
