# simple_web_app

## 1. 使用actix_web库实现Hello Wrold的Web程序
需要用到的知识：
- lib模块
- 设置环境变量
- struct、trait
- 闭包
- Result和Option

运行：
```bash
$ cargo run
```
另起一个终端执行：
```bash
$ curl localhost:8000  
{"message":"world"}

$ curl -H "hello: actix" localhost:8000
{"message":"actix"}”
```