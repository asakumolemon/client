use std::time::Duration;

use crate::{model::app_state::AppState};
use crate::util::request_util::*;
use crate::util::file_util::*;

use server::model::article::{Article, CommonArticleReq};
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
        // 从命令行读取用户名
        println!("请输入用户名:");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        // 从命令行读取密码
        println!("请输入密码:");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("读取密码失败");
        let password = password.trim().to_string();

        let req = CommonRequest {
            token: None,
            data: LoginReq {
                username,
                password,
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
                    println!("登录成功!");
                    sleep(Duration::from_secs(10)).await;
                }
                None => {
                    println!("登录失败");
                }
            }
        }
    }

    pub async fn register(&self) {
        // 从命令行读取用户名
        println!("请输入用户名:");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        // 从命令行读取密码
        println!("请输入密码:");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("读取密码失败");
        let password = password.trim().to_string();

        // 从命令行读取确认密码
        println!("请再次输入密码:");
        let mut confirm_password = String::new();
        std::io::stdin().read_line(&mut confirm_password).expect("读取确认密码失败");
        let confirm_password = confirm_password.trim().to_string();

        // 检查两次输入的密码是否一致
        if password != confirm_password {
            println!("两次输入的密码不一致");
            return;
        }

        let req = CommonRequest {
            token: None,
            data: RegisterReq {
                username,
                password,
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
        // 从命令行读取要升级的用户名
        println!("请输入要升级的用户名:");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        // 从命令行读取要升级的等级
        println!("请输入要升级的等级:");
        let mut safe_level = String::new();
        std::io::stdin().read_line(&mut safe_level).expect("读取等级失败");
        let safe_level: i32 = safe_level.trim().parse().expect("请输入有效的数字");

        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: UpgradeSafeLevelReq {
                username,
                safe_level,
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
        // 从命令行读取文章ID
        println!("请输入要查询的文章ID:");
        let mut article_id = String::new();
        std::io::stdin().read_line(&mut article_id).expect("读取文章ID失败");
        let article_id: i32 = article_id.trim().parse().expect("请输入有效的数字");

        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: Some(article_id),
                article_title: None,
                author_id: Some(1),
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id),
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

    pub async fn get_article_by_title(&self) {
        // 从命令行读取文章标题
        println!("请输入要查询的文章标题:");
        let mut article_title = String::new();
        std::io::stdin().read_line(&mut article_title).expect("读取文章标题失败");
        let article_title = article_title.trim().to_string();

        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: None,
                article_title: Some(article_title),
                author_id: None,
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id),
            },
        };
        let resp = self.request_util.get_article_by_title(req).await;
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

    pub async fn get_article_by_author_id(&self) {
        // 从命令行读取作者ID
        println!("请输入要查询的作者ID:");
        let mut author_id = String::new();
        std::io::stdin().read_line(&mut author_id).expect("读取作者ID失败");
        let author_id: i32 = author_id.trim().parse().expect("请输入有效的数字");

        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: None,
                article_title: None,
                author_id: Some(author_id),
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id)
            }
        };
        let resp = self.request_util.get_article_by_author_id(req).await;
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

    pub async fn get_my_article(&self) {
        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: None,
                article_title: None,
                author_id: Some(self.app_state.user_id),
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id),
            }
        };
        let resp = self.request_util.get_article_by_author_id(req).await;
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

    pub async fn create_article(&self) {
        // 从命令行读取文章标题
        println!("请输入文章标题:");
        let mut title = String::new();
        std::io::stdin().read_line(&mut title).expect("读取文章标题失败");
        let title = title.trim().to_string();

        // 从命令行读取文件名
        println!("请输入要读取的文件名:");
        let mut filename = String::new();
        std::io::stdin().read_line(&mut filename).expect("读取文件名失败");
        let filename = filename.trim().to_string();

        // 从命令行输入安全等级
        println!("请输入文章安全等级:");
        let mut safe_level = String::new();
        std::io::stdin().read_line(&mut safe_level).expect("读取安全等级失败");
        let safe_level: i32 = safe_level.trim().parse().expect("请输入有效的数字");

        if safe_level > self.app_state.safe_level {
            println!("安全等级不能大于当前用户等级");
            return;
        }

        let file_content = read_file(&filename);
        let content = Article {
            id: 0,
            title: title.clone(),
            content: file_content,
            author_id: self.app_state.user_id,
            safe_level: safe_level,
        };
        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: None,
                article_title: Some(title),
                author_id: Some(self.app_state.user_id),
                username: self.app_state.username.clone(),
                content: Some(content),
                user_id: Some(self.app_state.user_id),
            }
        };
        let resp = self.request_util.create_article(req).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                println!("创建文章成功");
            } else {
                println!("创建文章失败: {}", resp.msg);
            }
        }
    }

    pub async fn update_article(&self) {
        // 从命令行读取文章ID
        println!("请输入要更新的文章ID:");
        let mut article_id = String::new();
        std::io::stdin().read_line(&mut article_id).expect("读取文章ID失败");
        let article_id: i32 = article_id.trim().parse().expect("请输入有效的数字");

        // 从命令行读取文章标题
        println!("请输入新的文章标题:");
        let mut title = String::new();
        std::io::stdin().read_line(&mut title).expect("读取文章标题失败");
        let title = title.trim().to_string();

        // 从命令行读取文件名
        println!("请输入要读取的文件名:");
        let mut filename = String::new();
        std::io::stdin().read_line(&mut filename).expect("读取文件名失败");
        let filename = filename.trim().to_string();

        // 从命令行输入安全等级
        println!("请输入新的文章安全等级:");
        let mut safe_level = String::new();
        std::io::stdin().read_line(&mut safe_level).expect("读取安全等级失败");
        let safe_level: i32 = safe_level.trim().parse().expect("请输入有效的数字");

        if safe_level > self.app_state.safe_level {
            println!("安全等级不能大于当前用户等级");
            return;
        }

        let file_content = read_file(&filename);
        let content = Article {
            id: article_id,
            title: title.clone(),
            content: file_content,
            author_id: self.app_state.user_id,
            safe_level: safe_level,
        };

        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: Some(article_id),
                article_title: Some(title),
                author_id: Some(self.app_state.user_id),
                username: self.app_state.username.clone(),
                content: Some(content),
                user_id: Some(self.app_state.user_id)
            }
        };
        let resp = self.request_util.update_article(req).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                println!("更新文章成功");
            } else {
                println!("更新文章失败: {}", resp.msg);
            }
        }
    }

    pub async fn delete_article(&self) {
        // 从命令行读取文章ID
        println!("请输入要删除的文章ID:");
        let mut article_id = String::new();
        std::io::stdin().read_line(&mut article_id).expect("读取文章ID失败");
        let article_id: i32 = article_id.trim().parse().expect("请输入有效的数字");

        // 先获取文章信息
        let req_get = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: Some(article_id),
                article_title: None,
                author_id: None,
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id)
            }
        };
        
        // 展示文章信息并确认
        let resp = self.request_util.get_article_by_id(req_get).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                if let Some(data) = resp.data {
                    println!("将要删除的文章: {}", data.title);
                    println!("确认删除? (y/n):");
                    let mut confirm = String::new();
                    std::io::stdin().read_line(&mut confirm).expect("读取确认失败");
                    if confirm.trim().to_lowercase() != "y" {
                        println!("取消删除");
                        return;
                    }
                }
            } else {
                println!("获取文章失败: {}", resp.msg);
                return;
            }
        }

        let req_delete = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: Some(article_id),
                article_title: None,
                author_id: None,
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id)
            }
        };

        // 执行删除
        let resp = self.request_util.delete_article(req_delete).await;
        if let Ok(resp) = resp {
            if resp.code == 0 {
                println!("删除文章成功");
            } else {
                println!("删除文章失败: {}", resp.msg);
            }
        }
    }

    pub async fn fetch_one(&self) {
        let req = CommonRequest {
            token: Some(self.app_state.token.clone()),
            data: CommonArticleReq {
                article_id: None,
                article_title: None,
                author_id: None,
                username: self.app_state.username.clone(),
                content: None,
                user_id: Some(self.app_state.user_id)
            }
        };
        let resp = self.request_util.fetch_one(req).await;
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