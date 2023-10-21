use actix_web::HttpResponse;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;
use validator::ValidationErrors;

pub fn create_validation_errors_response(errors: ValidationErrors) -> HttpResponse {
    return HttpResponse::UnprocessableEntity()
        .json(serde_json::json!({
            "status": "error",
            "errors": errors
        }));
}
pub fn create_error_response(error_text: String) -> HttpResponse {
    return HttpResponse::InternalServerError()
        .json(serde_json::json!({
            "status": "error",
            "message": error_text
        }));
}

pub fn create_conflict_response(error_text: String) -> HttpResponse {
    return HttpResponse::Conflict()
        .json(serde_json::json!({
            "status": "fail",
            "message": error_text
        }));
}

pub fn get_password_hash(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    return Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
}
