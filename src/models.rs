use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseError {
    pub error: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseVersion {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub error: bool,
    pub data: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUsers {
    pub error: bool,
    pub count: usize,
    pub data: Vec<User>,
}
