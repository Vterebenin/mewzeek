use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Name should not be empty"))]
    pub name: String,
    #[validate(email(message = "Email should look like email"))]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password should not be empty"),
        must_match(
            other = "password_confirmation",
            message = "Passwords are not matching"
        )
    )]
    pub password: String,
    #[serde(rename = "passwordConfirmation")]
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
