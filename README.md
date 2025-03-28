# Satex

[![Build Status](https://github.com/w-sodalite/satex/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/w-sodalite/satex/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/satex)](https://crates.io/crates/satex)

一个轻量级、高性能的HTTP网关，使用Rust语言开发，项目灵感来自于`Spring Cloud Gateway`。

> NOTICE(🫡): v0.3版本将以前的代码完全重构，有一些v0.2的功能暂时还未完全实现，目前也在积极开发中。

## 初衷

因为学习Rust有一段时间了，但是平时上主要用的是Java，刚好最近维护的公司的网关(基于`Spring Cloud Gateway`)
有些性能问题，想着能不能使用Rust来开发一个和`Spring Cloud Gateway`差不多的网关。
然后就开发了这个项目。所以如果熟悉`Spring Cloud Gateway`，配置文件应该看着会比较熟悉。秉承能复用社区生态绝不重复造轮子，
所以项目中使用了大量开源的组件，比如`actix-net`、`rustls`、`hyper`等。(不得不说，rust的生态真不错!😍)

## 特性

- 异步IO
- thread-per-core线程模型
- TLS支持
- 动态路由
- 丰富的路由组件以及灵活的路由配置
- 兼容[tower](https://crates.io/crates/tower)和[tower-http](https://crates.io/crates/tower-http)的生态，包括中间件、服务和工具等。

## 实现

| 组件     | 实现                                                                                        |
|--------|-------------------------------------------------------------------------------------------|
| 异步运行时  | [tokio](https://github.com/tokio-rs/tokio)                                                |
| 网络     | [actix-net](https://github.com/actix/actix-net)                                           |
| TLS    | [rustls](https://github.com/rustls/rustls)                                                |
| HTTP协议 | [hyper](https://github.com/hyperium/hyper)                                                |
| 中间件    | [tower](https://crates.io/crates/tower) [tower-http](https://crates.io/crates/tower-http) |

## 运行

```shell
# 使用当前目录的satex.yaml文件
cargo run 
```

```shell
# 指定配置文件路径
cargo run -- -c ./satex.yaml
```

## 文档

TODO

## License

This project is licensed under the [MIT license](./LICENSE).