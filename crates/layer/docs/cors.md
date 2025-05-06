# Cors

CORS（跨域资源共享）中间件，用于在 Web 服务中配置跨域请求策略。

## 配置

| 参数名                   | 默认值                                        | 描述   |
|-----------------------|--------------------------------------------|------|
| max-age-secs          | 预检请求结果缓存时间（秒）                              | None |
| allow-credentials     | 是否允许携带凭证（如 Cookies）                        | None |
| allow-private-network | 是否允许访问私有网络资源                               | None |
| allow-headers         | 允许的客户端请求头（如 Content-Type）                  | None |
| allow-methods         | 允许的 HTTP 方法（如 GET, POST）                   | None |
| allow-origin          | 允许的来源域名（如 "https://client.com" 或 "*" 全局允许） | None |
| expose-headers        | 允许客户端访问的响应头（如自定义业务头 X-Total-Count）         | None |
| vary                  | 动态生成 Vary 响应头，根据请求头（如 Origin）调整 CORS 策略    | None |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: cors-full
      layers:
        - kind: Cors
          args:
            max-age-secs: 60
            allow-credentials: false
            allow-private-network: false
            allow-headers: '*'
            allow-methods: '*'
            allow-origin: '*'
            expose-headers: x-header1
```

- **快捷模式**

> 不支持(💔) 