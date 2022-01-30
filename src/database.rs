use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use crate::models;

lazy_static! {
    static ref DATABASE: Mutex<HashMap<Uuid, models::User>> = {
        let mut m = HashMap::new();
        let first_id = Uuid::new_v4();
        m.insert(first_id, models::User{
            id: first_id,
            name: "Edder Silva".to_string(),
            email: "arskang@gmail.com".to_string(),
        });
        Mutex::new(m)
    };
}

pub fn get_users() -> models::ResponseUsers {
    let map = DATABASE.lock().unwrap();
    let count = map.keys().len();
    let mut users: Vec<models::User> = Vec::new();
    for (_, value) in &*map {
        users.push(models::User {
            id: value.id,
            name: value.name.to_string(),
            email: value.email.to_string(),
        });
    }
    models::ResponseUsers {
        error: false,
        count: count,
        data: users,
    }
}

pub fn find_user(id: Uuid) -> Option<models::User> {
    let map = DATABASE.lock().unwrap();
    if map.contains_key(&id) {
        let user = map.get(&id);
        match user {
            Some(us) => return Some(models::User {
                id: us.id,
                name: us.name.to_string(),
                email: us.email.to_string(),
            }),
            None => return None
        }
    }
    None
}

pub fn create_user(user: &models::SimpleUser) -> Uuid {
    let id = Uuid::new_v4();
    let new_user = models::User {
        id,
        name: user.name.to_string(),
        email: user.email.to_string(),
    };
    let mut map = DATABASE.lock().unwrap();
    map.insert(id, new_user);
    id
}

pub fn update_user(user: &models::User) -> bool {
    let search_user = find_user(user.id);
    match search_user {
        Some(us) => {
            let mut map = DATABASE.lock().unwrap();
            *map.get_mut(&us.id).unwrap() = models::User {
                id: us.id,
                name: user.name.to_string(),
                email: user.email.to_string(),
            };
            return true
        }
        None => return false
    }
}

pub fn delete_user(id: Uuid) -> bool {
    let search_user = find_user(id);
    match search_user {
        Some(us) => {
            let mut map = DATABASE.lock().unwrap();
            map.remove(&us.id);
            return true
        },
        None => return false
    }
}