use simple_web_app::MessageApp;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); // 设置环境变量
    env_logger::init();
    let app = MessageApp::new(8000);
    app.run()
}
