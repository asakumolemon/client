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
    
}