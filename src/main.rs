use client::handler::app_handler::App;

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.login().await;
}
