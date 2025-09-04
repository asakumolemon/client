use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Clone)]
pub struct User { 
    pub id: i32,
    pub username: String,
    pub password: String,
    pub safe_level: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthUser { 
    pub username: String,
    pub password: String,
    pub safe_level: i32,
}

pub struct AuthUserResponse { 
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpgradeSafeLevelReq { 
    pub username: String,
    pub password: String,
    pub safe_level: i32
}

#[derive(Serialize, Deserialize)]
pub struct LoginReq { 
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterReq { 
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct CommonUserResp { 
    pub user_id: Option<i32>,
    pub safe_level: Option<i32>,
    pub username: String,
    pub token: String,
}