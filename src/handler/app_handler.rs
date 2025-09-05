use std::time::Duration;

use crate::{model::app_state::AppState};
use crate::util::request_util::*;
use server::model::article::CommonArticleReq;
use server::model::{common::*, user::*};
use tokio::time::sleep;
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

    pub async fn login(&mut self) {
        let req = CommonRequest {
            token: None,
            data: LoginReq {
                username: "asakumolemon2".to_string(),
                password: "bz987654321s".to_string(),
            },
        };
        let resp = self.request_util.login(req).await;
        if let Ok(resp) = resp {
            match resp.data {
                Some(data) => {
                    self.app_state.username = data.username.clone();
                    self.app_state.token = data.token.clone();
                    self.app_state.user_id = data.user_id.unwrap();
                    self.app_state.safe_level = data.safe_level.unwrap();
                    println!("Yay! Login success!");
                    sleep(Duration::from_secs(10)).await;
                }
                None => {
                    println!("Login failed");
                }
            }
            
        }
    }

    pub async fn register(&self) {
        let req = CommonRequest {
            token: None,
            data: RegisterReq {
                username: "asakumolemon2".to_string(),
                password: "bz987654321".to_string(),
            },
        };
        let resp = self.request_util.register(req).await;
        if let Ok(resp) = resp {    
            if resp.code == 0 {
                println!("注册成功");
            } else {
                println!("注册失败: {}", resp.msg);
            }
        }
    }

    pub async fn upgrade(&self) {
        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: UpgradeSafeLevelReq {
                username: self.app_state.username.clone(),
                safe_level: 1,
            },
        };
        let resp = self.request_util.upgrade_safe_level(req).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                println!("升级成功");
            } else {
                println!("升级失败: {}", resp.msg);
            }
        }
    }

    pub async fn get_article_by_id(&self) {
        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: Some(1),
                article_title: None,
                author_id: Some(1),
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(1),
            },
        };
        let resp = self.request_util.get_article_by_id(req).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                if let Some(data) = resp.data {
                    println!("{:?}", data);
                }
            } else {
                println!("获取文章失败: {}", resp.msg);
            }
        }
    }
}