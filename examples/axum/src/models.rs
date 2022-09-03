use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}
