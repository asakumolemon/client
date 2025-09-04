use crate::model::app_state::AppState;

pub struct HttpUtil {
    pub app_state: AppState,
    pub url_prefix: String,
}

impl HttpUtil {
    pub fn new(app_state: AppState) -> Self {
        let url_prefix = dotenvy::var("URL_PREFIX").unwrap_or("http://localhost:18080".to_string());
        Self {
            app_state,
            url_prefix,
        }
    }
}

impl HttpUtil {
    pub async fn get<T>(&self, url: &str) -> Result<T, reqwest::Error>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .send()
            .await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn post<T>(&self, url: &str, body: T) -> Result<T, reqwest::Error>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn put<T>(&self, url: &str, body: T) -> Result<T, reqwest::Error>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        let client = reqwest::Client::new();
        let resp = client
            .put(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn delete<T>(&self, url: &str, body: T) -> Result<T, reqwest::Error>
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        let client = reqwest::Client::new();
        let resp = client
            .delete(url)
            .json(&body)
            .send()
            .await?;
        let body = resp.json().await?;
        Ok(body)
    }
}