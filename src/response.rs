use chrono::prelude::*;
use serde::Serialize;
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Validate)]
pub struct FilteredUser {
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
