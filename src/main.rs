use client::handler::app_handler::App;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    
    // 显示欢迎信息
    println!("========================================");
    println!("欢迎使用文章管理系统");
    println!("========================================");
    
    // 登录流程
    loop {
        println!("\n请选择操作:");
        println!("1. 登录");
        println!("2. 注册");
        println!("0. 退出");
        print!("请输入选择: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        
        match choice.trim() {
            "1" => {
                app.login().await;
                if !app.app_state.token.is_empty() {
                    break;
                }
            }
            "2" => {
                app.register().await;
            }
            "0" => {
                println!("感谢使用，再见！");
                return;
            }
            _ => {
                println!("无效的选择，请重新输入！");
            }
        }
    }
    
    // 主操作循环
    loop {
        println!("\n========================================");
        println!("文章管理系统 - 主菜单");
        println!("当前用户: {} (安全等级: {})", app.app_state.username, app.app_state.safe_level);
        println!("========================================");
        println!("1. 创建文章");
        println!("2. 查看我的文章");
        println!("3. 根据ID查询文章");
        println!("4. 根据标题查询文章");
        println!("5. 根据作者ID查询文章");
        println!("6. 更新文章");
        println!("7. 删除文章");
        println!("8. 获取一篇随机文章");
        if app.app_state.safe_level >= 8 {
            println!("9. 升级用户安全等级");
        }
        println!("0. 退出系统");
        print!("请输入选择: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取输入失败");
        
        match choice.trim() {
            "1" => app.create_article().await,
            "2" => app.get_my_article().await,
            "3" => app.get_article_by_id().await,
            "4" => app.get_article_by_title().await,
            "5" => app.get_article_by_author_id().await,
            "6" => app.update_article().await,
            "7" => app.delete_article().await,
            "8" => app.fetch_one().await,
            "9" => {
                if app.app_state.safe_level >= 8 {
                    app.upgrade().await;
                } else {
                    println!("权限不足");
                }
            }
            "0" => {
                println!("感谢使用，再见！");
                break;
            }
            _ => {
                println!("无效的选择，请重新输入！");
            }
        }
    }
}
