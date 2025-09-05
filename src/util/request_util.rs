use server::model::{common::*, article::*,user::*};
use crate::util::http_util::HttpUtil;

pub struct RequestUtil {
    pub http_util: HttpUtil,
}

impl RequestUtil {
    pub fn new() -> Self {
        Self {
            http_util: HttpUtil::new(),
        }
    }

    pub async fn login(&self, req: CommonRequest<LoginReq>) -> Result<CommonResponse<CommonUserResp>, reqwest::Error> {
        let url = format!("{}/user/login", self.http_util.url_prefix);
        let resp: CommonResponse<CommonUserResp> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn register(&self, req: CommonRequest<RegisterReq>) -> Result<CommonResponse<CommonUserResp>, reqwest::Error> {
        let url = format!("{}/user/register", self.http_util.url_prefix);
        let resp: CommonResponse<CommonUserResp> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn upgrade_safe_level(&self, req: CommonRequest<UpgradeSafeLevelReq>) -> Result<CommonResponse<CommonUserResp>, reqwest::Error> {
        let url = format!("{}/user/upgrade", self.http_util.url_prefix);
        let resp: CommonResponse<CommonUserResp> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn get_article_by_id(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<Article>, reqwest::Error> {
        let url = format!("{}/article/get_by_id", self.http_util.url_prefix);
        let resp: CommonResponse<Article> = self.http_util.get(&url, req).await?;
        Ok(resp)
    }

    pub async fn get_article_by_title(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<Vec<Article>>, reqwest::Error> {
        let url = format!("{}/article/get_by_title", self.http_util.url_prefix);
        let resp: CommonResponse<Vec<Article>> = self.http_util.get(&url, req).await?;
        Ok(resp)
    }

    pub async fn get_article_by_author_id(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<Vec<Article>>, reqwest::Error> {
        let url = format!("{}/article/get_by_author_id", self.http_util.url_prefix);
        let resp: CommonResponse<Vec<Article>> = self.http_util.get(&url, req).await?;
        Ok(resp)
    }

    pub async fn create_article(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<i32>, reqwest::Error> {
        let url = format!("{}/article/create", self.http_util.url_prefix);
        let resp: CommonResponse<i32> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn update_article(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<()>, reqwest::Error> {
        let url = format!("{}/article/update", self.http_util.url_prefix);
        let resp: CommonResponse<()> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn delete_article(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<()>, reqwest::Error> {
        let url = format!("{}/article/delete", self.http_util.url_prefix);
        let resp: CommonResponse<()> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }

    pub async fn fetch_one(&self, req: CommonRequest<CommonArticleReq>) -> Result<CommonResponse<Article>, reqwest::Error> {
        let url = format!("{}/article/fetch_one", self.http_util.url_prefix);
        let resp: CommonResponse<Article> = self.http_util.post(&url, req).await?;
        Ok(resp)
    }
}