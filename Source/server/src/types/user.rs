use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub id: String,
    pub username: String,
    
    pub name: String,
    pub email: String,
    
    pub password: String,
}