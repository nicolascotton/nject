use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

