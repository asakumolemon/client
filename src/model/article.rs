use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Clone)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub safe_level: i32,
}


#[derive(Deserialize, Serialize)]
pub struct CommonArticleReq {
    pub article_id: Option<i32>,
    pub article_title: Option<String>,
    pub author_id: Option<i32>,
    pub username: String,
    pub content: Option<Article>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateArticleReq {
    pub username: String,
    pub content: Article,
}