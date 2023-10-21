use actix_web::HttpResponse;

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
