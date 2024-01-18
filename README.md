# Satex

使用`RUST`开发的轻量、高性能的HTTP网关，基于`tokio`、`hyper`、`tower`构建。

[![Build Status](https://github.com/w-sodalite/satex/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/w-sodalite/satex/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/satex)](https://crates.io/crates/satex)

## 优势

- 纯异步IO实现
- 丰富的路由组件以及灵活的路由配置
- 内置服务发现、健康检查以及多种负载均衡策略
- 完全兼容[tower](https://crates.io/crates/tower)和[tower-http](https://crates.io/crates/tower-http)的生态，包括中间件、服务和工具等。

## 路由结构

- **matcher**：负责接收并解析进入的HTTP请求。通过精确匹配算法，它能快速地识别出请求的目标路径、方法以及其他相关参数。

- **layer**：只有通过`matcher`匹配的请求才会进入`layer`
  层，主要对请求进行增强或过滤。你可以在这一层添加各种过滤逻辑，例如权限验证、限流、日志记录等。它确保只有满足特定条件的请求能够继续传递。

- **service**：负责处理经过`layer`层筛选的请求。你可以在这一层实现具体的业务逻辑，例如调用后端服务、处理数据等。

## 内置组件

- 路由组件

    - **Matcher**
        * `Method`：根据请求方法（如GET、POST等）进行匹配。
        * `Query`：根据请求的查询参数进行匹配。
        * `Header`：根据请求头信息进行匹配。
        * `Host`：根据请求的主机名进行匹配。
        * `Path`：根据请求路径进行匹配，使用[`path-tree`](https://docs.rs/path-tree)进行处理，支持变量绑定。
        * `RemoteAddr`：根据客户端的IP地址进行匹配。
        * `Cookie`: 根据请求的Cookie进行匹配。
        * `Time`：根据请求时间进行匹配。

    - **Layer**
        * `Cors`：处理跨域请求。
        * `KeepHostHeader`：保持原始的Host请求头。
        * `PathStrip`：从请求路径中删除或替换特定部分。
        * `RateLimit`：限制来自单个IP的请求频率。
        * `SetHeader`：设置请求头和响应头信息。
        * `ConcurrentcyLimit`：限制同时处理的请求数量。
        * `RequestBodyLimit`：限制请求体的最大大小。
        * `SetStatus`：设置响应状态码。
        * `RewritePath`: 重写请求的接口地址，支持使用`Path`匹配路由绑定的变量。

    - **Service**
        * `Echo`：简单地返回接收到的请求内容。
        * `Static`：提供静态文件服务。
        * `Proxy`：反向代理服务，代理请求到另一个服务或地址。


- 反向代理支持

    - 服务发现(Discovery)
        * `Builtin`：内置的服务发现，通过配置的方式注册服务集合，内部会定时检测服务节点的可用性。

    - 负载均衡(LoadBalance)
        * `IpHash`：IP哈希负载策略使用客户端的IP地址进行哈希计算，根据哈希值将请求分配给后端服务器。这样可以确保来自同一IP地址的请求始终被发送到同一台服务器，这有助于保持会话和状态信息的持续性。
        * `Random`：随机负载策略随机选择一台服务器将请求发送过去。这种策略简单且易于实现，适用于没有特殊需求的情况。
        * `Sequential`：顺序负载策略按照服务器列表的顺序依次将请求发送过去。这种策略适用于服务器性能基本一致的情况。
        * `StandBy`：备用负载策略在主服务器故障时，将请求切换到备用服务器上。这种策略可以提高系统的可用性和可靠性。
        * `Weight`：权重负载策略根据服务器的性能或权重值来分配请求。权重值高的服务器将获得更多的请求，而权重值低的服务器将获得较少的请求。这种策略可以帮助平衡服务器的负载，提高系统的性能和效率。

## 配置示例

[examples](./examples)

## 构建和启动

- ### 构建

    - 安装`RUST`环境，按照[官方文档](https://www.rust-lang.org/zh-CN/learn/get-started)初始化环境。

    - 使用`cargo`安装，执行命令：`cargo install satex`即可安装最新版本的`satex`。

    - 或者使用源码进行构建，下载源码到本地，在根目录执行`cargo build --release`，编译成功后在`target/release`
      目录下可以找到`satex`。

- ### 启动

  > 使用`-c`指定配置文件`satex.yaml`的路径

    ```shell
    satex -c exmaples/satex.yaml
    ```

## License

This project is licensed under the [Apache 2.0](./LICENSE)
