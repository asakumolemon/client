use client::handler::app_handler::App;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = App::new();
    let resp = app.login().await;
    println!("{:?}", resp);
}
