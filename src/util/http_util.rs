use server::model::{common::*};

pub struct HttpUtil {
    pub url_prefix: String,
}

impl HttpUtil {
    pub fn new() -> Self {
        let url_prefix = dotenvy::var("URL_PREFIX").unwrap_or("http://localhost:18080".to_string());
        Self {
            url_prefix,
        }
    }
}

impl HttpUtil {
    pub async fn get<T,R>(&self, url: &str, params: T) -> Result<CommonResponse<R>, reqwest::Error>
    where
        T: serde::Serialize ,
        R: for <'a> serde::Deserialize<'a>,
    {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .query(&params)
            .send()
            .await?;
        let body = resp.text().await?;
        let body: CommonResponse<R> = serde_json::from_str(&body).unwrap();
        Ok(body)
    }

    pub async fn post<T, R>(&self, url: &str, body: T) -> Result<CommonResponse<R>, reqwest::Error>
    where
        T: serde::Serialize,
        R: for<'a> serde::Deserialize<'a>,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.text().await?;
        let body: CommonResponse<R> = serde_json::from_str(&body).unwrap();
        Ok(body)
    }

    pub async fn put<T,R>(&self, url: &str, body: T) -> Result<CommonResponse<R>, reqwest::Error>
    where
        T: serde::Serialize + for <'a> serde::Deserialize <'a>,
        R: serde::Serialize + for <'a> serde::Deserialize <'a>,
    {
        let client = reqwest::Client::new();
        let resp = client
            .put(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.text().await?;
        let body: CommonResponse<R> = serde_json::from_str(&body).unwrap();
        Ok(body)
    }

    pub async fn delete<T,R>(&self, url: &str, body: T) -> Result<CommonResponse<R>, reqwest::Error>
    where
        T: serde::Serialize + for <'a> serde::Deserialize <'a>,
        R: serde::Serialize + for <'a> serde::Deserialize <'a>,
    {
        let client = reqwest::Client::new();
        let resp = client
            .delete(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.text().await?;
        let body: CommonResponse<R> = serde_json::from_str(&body).unwrap();
        Ok(body)        
    }
}