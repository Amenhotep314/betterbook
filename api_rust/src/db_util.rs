use crate::entity::user::Entity as User;


#[derive(Clone)]
pub struct CurrentUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

pub fn retrieve_user_by_email(email: &str) -> Option<CurrentUser> {
    // TODO: Implement database query here
}
