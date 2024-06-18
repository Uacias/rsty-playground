use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    id: Thing,
    pub name: String,
    pub email: String,
}
