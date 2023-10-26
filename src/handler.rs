use crate::{
    helpers::{
        create_conflict_response, create_error_response, create_validation_errors_response,
        get_password_hash,
    },
    jwt_auth,
    model::{LoginUserSchema, RegisterUserSchema, TokenClaims, User, NewUser},
    response::FilteredUser,
    AppState,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::{QueryDsl, ExpressionMethods};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use validator::Validate;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::schema::users;

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let body_items = body.clone();

    match body_items.validate() {
        Ok(_) => (),
        Err(e) => return create_validation_errors_response(e),
    };
    let existed_users = users
        .filter(email.eq(body.email.to_owned()))
        .select(User::as_select())
        .load(&mut data.db.get().unwrap())
        .expect("Error fetching user");

    // todo: probably should be in validator
    if !existed_users.is_empty() {
        return create_conflict_response("email_conflict".to_string());
    }

    let hashed_password = get_password_hash(&body.password);
    let user = NewUser {
        name: body.name.to_string(),
        email: body.email.to_string().to_lowercase(),
        password: hashed_password,
    };
    let query_result = diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(&mut data.db.get().unwrap());

    match query_result {
        Ok(user) => HttpResponse::Ok().json(filter_user_record(&user)),
        Err(e) => create_error_response(e.to_string()),
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = users
        .filter(email.eq(body.email.to_owned()))
        .select(User::as_select())
        .first(&mut data.db.get().unwrap())
        .optional()
        .expect("Error fetching user");

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "incorrect_email_or_password"}));
    }
    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    _: HttpRequest,
    _: web::Data<AppState>,
    auth: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let user = auth.user;

    let json_response = serde_json::json!(filter_user_record(&user));

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
