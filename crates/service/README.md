# satex-service

Service 模块用于处理请求，并返回响应。

## 内置模块

| 名称         | 描述                                       | 文档                             |
|------------|------------------------------------------|--------------------------------|
| `Echo`     | Echo 模块用于返回指定内容，通常用于测试、开发环境。             | [README.md](docs/echo.md)      |
| `ServeDir` | ServeDir 模块用于提供静态文件服务，支持目录的递归遍历和文件缓存。    | [README.md](docs/serve_dir.md) |
| `Proxy`    | Proxy 模块用于代理请求到目标服务器，通常用于实现负载均衡、反向代理等场景。 | [README.md](docs/proxy.md)     |
