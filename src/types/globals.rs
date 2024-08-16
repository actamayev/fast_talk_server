use crate::entities::credentials::Model as User;

pub struct CredentialsData {
    pub username: String,
    pub hashed_password: String,
    pub email: String
}

pub enum EmailOrUsername {
    Email,
    Username
}

// Define a wrapper struct for the authenticated user
#[derive(Clone)]
pub struct AuthenticatedUser(pub User); // User is your user model type

#[derive(Clone)]
// Define a wrapper struct for the friend user
pub struct FriendUser(pub User);
