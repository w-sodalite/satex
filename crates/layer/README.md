# satex-layer

Layer 模块用于实现 HTTP 请求和响应的中间件处理，提供了一组可插拔、可组合的功能组件（Layer），在请求到达服务或响应返回客户端之前进行拦截并执行某些操作。

## 内置组件

| 名称                     | 描述                                   | 文档                                          |
|------------------------|--------------------------------------|---------------------------------------------|
| `Cors`                 | CORS（跨域资源共享）中间件，用于在 Web 服务中配置跨域请求策略。 | [README.md](docs/cors.md)                   |
| `SetPrefix`            | 路径前缀设置中间件，用于设置请求路径的起始部分。             | [README.md](docs/set_prefix.md)             |
| `StripPrefix`          | 路径前缀剥离中间件，用于自动移除请求路径中的指定层级前缀。        | [README.md](docs/strip_prefix.md)           |
| `SetMethod`            | 设置请求方法中间件，用于在请求到达服务之前设置请求方法。         | [README.md](docs/set_method.md)             |
| `SetStatus`            | 设置状态码中间件，用于在响应到达客户端之前设置状态码。          | [README.md](docs/set_status.md)             |
| `Timeout`              | 设置请求超时时间中间件，用于在请求到达服务之前设置请求超时时间。     | [README.md](docs/timeout.md)                |
| `ConcurrencyLimit`     | 并发限制中间件，用于在请求到达服务之前设置并发限制。           | [README.md](docs/concurrency_limit.md)      |
| `SetRequestHeader`     | 请求头中间件，用于在请求到达服务之前设置请求头。             | [README.md](docs/set_request_header.md)     |
| `SetResponseHeader`    | 响应头中间件，用于在响应到达客户端之前设置响应头。            | [README.md](docs/set_response_header.md)    |
| `RemoveRequestHeader`  | 请求头中间件，用于在请求到达服务之前移除指定的请求头。          | [README.md](docs/remove_request_header.md)  |
| `RemoveResponseHeader` | 响应头中间件，用于在响应到达客户端之前移除指定的响应头。         | [README.md](docs/remove_response_header.md) |
