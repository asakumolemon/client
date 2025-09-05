use crate::{model::app_state::AppState};
use crate::util::request_util::*;
use server::model::{common::*, user::*};
pub struct App{
    pub app_state: AppState,
    pub request_util: RequestUtil,
}

impl App {
    pub fn new() -> Self {
        let app_state = AppState::new();
        let request_util = RequestUtil::new();
        Self { app_state, request_util }
    }

    pub async fn login(&self) -> CommonResponse<CommonUserResp> {
        let req = CommonRequest {
            token: None,
            data: LoginReq {
                username: "asakumolemon2".to_string(),
                password: "bz987654321s".to_string(),
            },
        };
        let resp = self.request_util.login(req).await.unwrap();
        resp
    }

    pub async fn register(&self) -> CommonResponse<CommonUserResp> {
        let req = CommonRequest {
            token: None,
            data: RegisterReq {
                username: "asakumolemon2".to_string(),
                password: "bz987654321s".to_string(),
            },
        };
        let resp = self.request_util.register(req).await.unwrap();
        resp
    }
}