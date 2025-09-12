use std::io::Write;

use crate::{model::app_state::AppState};
use crate::util::request_util::*;
use crate::util::file_util::*;

use server::model::article::{Article, CommonArticleReq};
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

    pub fn display_header(&self, title: &str) {
        println!("\n========================================");
        println!("{}", title);
        println!("========================================");
    }

    pub async fn login(&mut self) {
        self.display_header("用户登录");
        
        // 从命令行读取用户名
        print!("请输入用户名: ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        if username.is_empty() {
            println!("用户名不能为空！");
            return;
        }

        // 从命令行读取密码
        print!("请输入密码: ");
        std::io::stdout().flush().unwrap();
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("读取密码失败");
        let password = password.trim().to_string();

        if password.is_empty() {
            println!("密码不能为空！");
            return;
        }

        let req = CommonRequest {
            token: None,
            data: LoginReq {
                username,
                password,
            },
        };
        
        println!("正在登录...");
        let resp = self.request_util.login(req).await;
        if let Ok(resp) = resp {
            match resp.data {
                Some(data) => {
                    self.app_state.username = data.username.clone();
                    self.app_state.token = data.token.clone();
                    self.app_state.user_id = data.user_id.unwrap();
                    self.app_state.safe_level = data.safe_level.unwrap();
                    println!("✅ 登录成功！欢迎，{}！", data.username);
                }
                None => {
                    println!("❌ 登录失败：用户名或密码错误");
                }
            }
        } else {
            println!("❌ 登录失败：网络错误或服务器无响应");
        }
    }

    pub async fn register(&self) {
        self.display_header("用户注册");
        
        // 从命令行读取用户名
        print!("请输入用户名: ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        if username.is_empty() {
            println!("用户名不能为空！");
            return;
        }

        if username.len() < 3 {
            println!("用户名至少需要3个字符！");
            return;
        }

        // 从命令行读取密码
        print!("请输入密码: ");
        std::io::stdout().flush().unwrap();
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("读取密码失败");
        let password = password.trim().to_string();

        if password.is_empty() {
            println!("密码不能为空！");
            return;
        }

        if password.len() < 6 {
            println!("密码至少需要6个字符！");
            return;
        }

        // 从命令行读取确认密码
        print!("请再次输入密码: ");
        std::io::stdout().flush().unwrap();
        let mut confirm_password = String::new();
        std::io::stdin().read_line(&mut confirm_password).expect("读取确认密码失败");
        let confirm_password = confirm_password.trim().to_string();

        // 检查两次输入的密码是否一致
        if password != confirm_password {
            println!("❌ 两次输入的密码不一致！");
            return;
        }

        let req = CommonRequest {
            token: None,
            data: RegisterReq {
                username,
                password,
            },
        };
        
        println!("正在注册...");
        let resp = self.request_util.register(req).await;
        if let Ok(resp) = resp {    
            if resp.code == 0 {
                println!("✅ 注册成功！请登录您的账户。");
            } else {
                println!("❌ 注册失败: {}", resp.msg);
            }
        } else {
            println!("❌ 注册失败：网络错误或服务器无响应");
        }
    }

    pub async fn upgrade(&self) {
        self.display_header("升级用户安全等级");
        
        // 从命令行读取要升级的用户名
        print!("请输入要升级的用户名: ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).expect("读取用户名失败");
        let username = username.trim().to_string();

        if username.is_empty() {
            println!("用户名不能为空！");
            return;
        }

        // 从命令行读取要升级的等级
        print!("请输入要升级的等级 (1-10): ");
        std::io::stdout().flush().unwrap();
        let mut safe_level = String::new();
        std::io::stdin().read_line(&mut safe_level).expect("读取等级失败");
        
        let safe_level: i32 = match safe_level.trim().parse() {
            Ok(level) => {
                if level < 1 || level > 10 {
                    println!("等级必须在1-10之间！");
                    return;
                }
                level
            }
            Err(_) => {
                println!("请输入有效的数字！");
                return;
            }
        };

        println!("正在升级用户 {} 的安全等级到 {}...", username, safe_level);
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
                println!("✅ 升级成功！用户 {} 的安全等级已提升到 {}", 
                         resp.data.as_ref().unwrap().username, 
                         resp.data.as_ref().unwrap().safe_level.unwrap_or(0));
            } else {
                println!("❌ 升级失败: {}", resp.msg);
            }
        } else {
            println!("❌ 升级失败：网络错误或服务器无响应");
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
                    println!("\n文章详情:");
                    println!("ID: {}", data.id);
                    println!("标题: {}", data.title);
                    println!("作者ID: {}", data.author_id);
                    println!("安全等级: {}", data.safe_level);
                    
                    println!("\n请选择查看方式:");
                    println!("1. 直接查看内容");
                    println!("2. 保存到本地文件");
                    print!("请输入选择: ");
                    std::io::stdout().flush().unwrap();
                    
                    let mut choice = String::new();
                    std::io::stdin().read_line(&mut choice).expect("读取输入失败");
                    
                    match choice.trim() {
                        "1" => {
                            println!("内容:\n{}", data.content);
                        }
                        "2" => {
                            let filename = format!("article_{}_{}.txt", data.id, data.title.replace(' ', "_"));
                            let content = format!("标题: {}\n作者ID: {}\n安全等级: {}\n\n内容:\n{}", 
                                data.title, data.author_id, data.safe_level, data.content);
                            
                            crate::util::file_util::write_file(&filename, &content);
                            println!("文章已保存到文件: {}", filename);
                        }
                        _ => {
                            println!("无效的选择，默认直接查看内容:");
                            println!("内容:\n{}", data.content);
                        }
                    }
                } else {
                    println!("未找到相关文章");
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
                if let Some(articles) = resp.data {
                    if articles.is_empty() {
                        println!("未找到相关文章");
                        return;
                    }

                    println!("找到以下文章:");
                    for (index, article) in articles.iter().enumerate() {
                        println!("{}. ID: {}, 标题: {}", index + 1, article.id, article.title);
                    }

                    println!("请选择要查看的文章序号 (输入0取消):");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("读取选择失败");
                    
                    if let Ok(choice) = input.trim().parse::<usize>() {
                        if choice == 0 {
                            println!("取消查看");
                            return;
                        }
                        
                        if choice > 0 && choice <= articles.len() {
                            let selected_article = &articles[choice - 1];
                            println!("\n文章详情:");
                            println!("ID: {}", selected_article.id);
                            println!("标题: {}", selected_article.title);
                            println!("作者ID: {}", selected_article.author_id);
                            println!("安全等级: {}", selected_article.safe_level);
                            
                            println!("\n请选择查看方式:");
                            println!("1. 直接查看内容");
                            println!("2. 保存到本地文件");
                            print!("请输入选择: ");
                            std::io::stdout().flush().unwrap();
                            
                            let mut view_choice = String::new();
                            std::io::stdin().read_line(&mut view_choice).expect("读取输入失败");
                            
                            match view_choice.trim() {
                                "1" => {
                                    println!("内容:\n{}", selected_article.content);
                                }
                                "2" => {
                                    let filename = format!("article_{}_{}.txt", selected_article.id, selected_article.title.replace(' ', "_"));
                                    let content = format!("标题: {}\n作者ID: {}\n安全等级: {}\n\n内容:\n{}", 
                                        selected_article.title, selected_article.author_id, selected_article.safe_level, selected_article.content);
                                    
                                    crate::util::file_util::write_file(&filename, &content);
                                    println!("文章已保存到文件: {}", filename);
                                }
                                _ => {
                                    println!("无效的选择，默认直接查看内容:");
                                    println!("内容:\n{}", selected_article.content);
                                }
                            }
                        } else {
                            println!("无效的选择");
                        }
                    } else {
                        println!("请输入有效的数字");
                    }
                } else {
                    println!("未找到相关文章");
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
                if let Some(articles) = resp.data {
                    if articles.is_empty() {
                        println!("该作者没有发布任何文章");
                        return;
                    }

                    println!("作者的文章列表:");
                    for (index, article) in articles.iter().enumerate() {
                        println!("{}. ID: {}, 标题: {}", index + 1, article.id, article.title);
                    }

                    println!("请选择要查看的文章序号 (输入0取消):");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("读取选择失败");
                    
                    if let Ok(choice) = input.trim().parse::<usize>() {
                        if choice == 0 {
                            println!("取消查看");
                            return;
                        }
                        
                        if choice > 0 && choice <= articles.len() {
                            let selected_article = &articles[choice - 1];
                            println!("\n文章详情:");
                            println!("ID: {}", selected_article.id);
                            println!("标题: {}", selected_article.title);
                            println!("作者ID: {}", selected_article.author_id);
                            println!("安全等级: {}", selected_article.safe_level);
                            
                            println!("\n请选择查看方式:");
                            println!("1. 直接查看内容");
                            println!("2. 保存到本地文件");
                            print!("请输入选择: ");
                            std::io::stdout().flush().unwrap();
                            
                            let mut view_choice = String::new();
                            std::io::stdin().read_line(&mut view_choice).expect("读取输入失败");
                            
                            match view_choice.trim() {
                                "1" => {
                                    println!("内容:\n{}", selected_article.content);
                                }
                                "2" => {
                                    let filename = format!("article_{}_{}.txt", selected_article.id, selected_article.title.replace(' ', "_"));
                                    let content = format!("标题: {}\n作者ID: {}\n安全等级: {}\n\n内容:\n{}", 
                                        selected_article.title, selected_article.author_id, selected_article.safe_level, selected_article.content);
                                    
                                    crate::util::file_util::write_file(&filename, &content);
                                    println!("文章已保存到文件: {}", filename);
                                }
                                _ => {
                                    println!("无效的选择，默认直接查看内容:");
                                    println!("内容:\n{}", selected_article.content);
                                }
                            }
                        } else {
                            println!("无效的选择");
                        }
                    } else {
                        println!("请输入有效的数字");
                    }
                } else {
                    println!("该作者没有发布任何文章");
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
                if let Some(articles) = resp.data {
                    if articles.is_empty() {
                        println!("您还没有发布任何文章");
                        return;
                    }

                    println!("我的文章列表:");
                    for (index, article) in articles.iter().enumerate() {
                        println!("{}. ID: {}, 标题: {}", index + 1, article.id, article.title);
                    }

                    println!("请选择要查看的文章序号 (输入0取消):");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("读取选择失败");
                    
                    if let Ok(choice) = input.trim().parse::<usize>() {
                        if choice == 0 {
                            println!("取消查看");
                            return;
                        }
                        
                        if choice > 0 && choice <= articles.len() {
                            let selected_article = &articles[choice - 1];
                            println!("\n文章详情:");
                            println!("ID: {}", selected_article.id);
                            println!("标题: {}", selected_article.title);
                            println!("作者ID: {}", selected_article.author_id);
                            println!("安全等级: {}", selected_article.safe_level);
                            
                            println!("\n请选择查看方式:");
                            println!("1. 直接查看内容");
                            println!("2. 保存到本地文件");
                            print!("请输入选择: ");
                            std::io::stdout().flush().unwrap();
                            
                            let mut view_choice = String::new();
                            std::io::stdin().read_line(&mut view_choice).expect("读取输入失败");
                            
                            match view_choice.trim() {
                                "1" => {
                                    println!("内容:\n{}", selected_article.content);
                                }
                                "2" => {
                                    let filename = format!("article_{}_{}.txt", selected_article.id, selected_article.title.replace(' ', "_"));
                                    let content = format!("标题: {}\n作者ID: {}\n安全等级: {}\n\n内容:\n{}", 
                                        selected_article.title, selected_article.author_id, selected_article.safe_level, selected_article.content);
                                    
                                    crate::util::file_util::write_file(&filename, &content);
                                    println!("文章已保存到文件: {}", filename);
                                }
                                _ => {
                                    println!("无效的选择，默认直接查看内容:");
                                    println!("内容:\n{}", selected_article.content);
                                }
                            }
                        } else {
                            println!("无效的选择");
                        }
                    } else {
                        println!("请输入有效的数字");
                    }
                } else {
                    println!("您还没有发布任何文章");
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
            title: filename.split('.').next().unwrap().to_string(),
            content: file_content,
            author_id: self.app_state.user_id,
            file_type: filename.split('.').last().unwrap().to_string(),
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
            title: filename.split('.').next().unwrap().to_string(),
            content: file_content,
            author_id: self.app_state.user_id,
            file_type: filename.split('.').last().unwrap().to_string(),
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