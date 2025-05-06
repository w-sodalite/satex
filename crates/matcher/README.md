# satex-matcher

Matcher 模块用于根据请求的某些特征（如时间、Cookie、HeaderHost、方法、路径、查询参数、远程地址等）来匹配特定条件，通常用于路由规则或中间件配置中，以决定是否对请求应用某种处理逻辑。

## 内置组件

| 名称           | 描述                                                                   | 文档                               |
|--------------|----------------------------------------------------------------------|----------------------------------|
| `After`      | After 匹配组件，用于判断请求是否在指定时间之后。                                          | [README.md](docs/after.md)       |
| `Before`     | Before 匹配组件，用于判断请求是否在指定时间之前。                                         | [README.md](docs/before.md)      |
| `Between`    | Between 匹配组件，用于判断请求是否在指定区间内。                                         | [README.md](docs/between.md)     |
| `Cookie`     | Cookie 匹配组件，用于根据请求头中的Cookie来匹配请求。                                    | [README.md](docs/cookie.md)      |
| `Host`       | Host 匹配组件，用于根据请求头中的Host来匹配请求。                                        | [README.md](docs/host.md)        |
| `Header`     | Header 匹配组件，用于根据请求头来匹配请求。                                            | [README.md](docs/header.md)      |
| `Method`     | Method 匹配组件，用于根据请求的方法（即 HTTP 请求中的方法，如 GET、POST、PUT、DELETE 等）来匹配特定条件。 | [README.md](docs/method.md)      |
| `Path`       | Path 匹配组件，用于根据请求的路径（即 URL 中的路径部分）来匹配特定条件。                            | [README.md](docs/path.md)        |
| `Query`      | Query 是一个查询参数匹配组件，用于根据请求的查询参数（即 URL 中的参数）来匹配特定条件。                    | [README.md](docs/query.md)       |
| `RemoteAddr` | 远程地址匹配组件，用于根据请求的远程地址（即客户端的 IP 地址）来匹配特定条件。                            | [README.md](docs/remote_addr.md) |