use actix_web::{get, post, put, delete, web, HttpResponse};
use uuid::Uuid;
use crate::helpers;
use crate::models;
use crate::database;

// Versión
#[get("/version")]
async fn version() -> HttpResponse {
    let version: models::ResponseVersion = models::ResponseVersion {
        version: "[1.0.0]".to_string(),
    };
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(version)
}

// Obtener usuarios
#[get("/user")]
async fn get_users() -> HttpResponse {
    let respose = database::get_users();
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(respose)
}

// Obtener usuario por ID
#[get("/user/{id}")]
async fn get_user(web::Path(id): web::Path<Uuid>) -> HttpResponse {
    let user = database::find_user(id);
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(models::ResponseUser{
            error: helpers::has_user(&user),
            data: user,
        })
}

// Crear usuario
#[post("/user")]
async fn create_user(body: web::Json<models::SimpleUser>) -> HttpResponse {
    let user: models::SimpleUser = models::SimpleUser {
        email: body.email.to_string(),
        name: body.name.to_string(),
    };
    let id = database::create_user(&user);
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(models::ResponseUser{
            error: false,
            data: Some(models::User {
                id: id,
                name: user.name,
                email: user.email,
            }),
        })
}

// Actualizar usuario
#[put("/user")]
async fn update_user(body: web::Json<models::User>) -> HttpResponse {
    let user: models::User = models::User {
        id: body.id,
        email: body.email.to_string(),
        name: body.name.to_string(),
    };
    let has_updated = database::update_user(&user);
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(models::ResponseUser{
            error: !has_updated,
            data: if has_updated { Some(user) } else { None },
        })
}

// Eliminar usuario
#[delete("/user/{id}")]
async fn delete_user(web::Path(id): web::Path<Uuid>) -> HttpResponse {
    let has_delete = database::delete_user(id);
    HttpResponse::Ok()
        .content_type(helpers::CONTENT_TYPE)
        .json(models::ResponseError{
            error: has_delete,
            message: if has_delete { "Usuario eliminado".to_string() } else { "El usuario no pudo ser eliminado, es posible que no exista o ya haya sido eliminado".to_string() },
        })
}

// Manejo de errores
#[get("*")]
async fn get_error_404() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(helpers::CONTENT_TYPE)
        .json(helpers::set_error_404())
}

// Manejo de errores
#[post("*")]
async fn post_error_404() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(helpers::CONTENT_TYPE)
        .json(helpers::set_error_404())
}

// Manejo de errores
#[put("*")]
async fn put_error_404() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(helpers::CONTENT_TYPE)
        .json(helpers::set_error_404())
}

// Manejo de errores
#[delete("*")]
async fn delete_error_404() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(helpers::CONTENT_TYPE)
        .json(helpers::set_error_404())
}