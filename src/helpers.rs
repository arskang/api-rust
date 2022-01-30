use crate::models;

pub const CONTENT_TYPE: &str = "application/json; charset=utf-8";

pub fn set_error_404() -> models::ResponseError {
    return models::ResponseError {
        error: true,
        message: "La ruta no se encuentra disponible".to_string(),
    };
}

pub fn has_user(user: &Option<models::User>) -> bool {
    match user {
        Some(_) => true,
        None => false,
    }
}