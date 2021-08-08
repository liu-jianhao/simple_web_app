#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    // 构造函数
    pub fn new(port: u16) -> Self {
        MessageApp { port } // 简洁写法
    }

    // 不可变借用
    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        // 闭包，move关键字用来修饰闭包，意味着闭包拥有这些变量的所有权
        HttpServer::new(move || {
            App::new()
            .wrap(middleware::Logger::default())
            .service(index)
        })
        // '?'的作用是如果返回Err就提前返回，否则继续走下去
        // 相当于:
        // if result.is_err() {
        //     return Err(result.err().unwrap());
        // }
        // result.unwrap().workers(8).run()
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

// actix提供的属性
#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok()) // 该方法需要返回Option，.ok()将Result变成Option
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(), // hello是&str类型
    }))
}