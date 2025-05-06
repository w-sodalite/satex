# Proxy

Proxy 模块用于代理请求到目标服务器，通常用于实现负载均衡、反向代理等场景。

## 配置

| 参数名    | 默认值 | 描述             |
|--------|-----|----------------|
| url    |     | 目标服务器地址。       |
| client |     | 反向代理HTTP客户端配置。 |

`Client`

| 参数名                    | 默认值 | 描述             |
|------------------------|-----|----------------|
| pool_max_idle_per_host |     | 目标服务器地址。       |
| pool_idle_timeout_secs |     | 反向代理HTTP客户端配置。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: proxy-full
      matchers:
        - kind: Proxy
          args:
            url: http://127.0.0.1:8080
            client:
              pool_idle_timeout_secs: 30
              pool_max_idle_per_host: 100
```

- **快捷配置**

```yaml
router:
  routes:
    - id: proxy-full
      matchers:
        - Proxy=http://127.0.0.1:8080
```

- **负载均衡**

```yaml
resolvers:
  - kind: Static
    args:
      upstreams:
        - name: backend-1
          policy: RoundRobin
          health-check:
            enabled: true
          addrs:
            - 127.0.0.1:8080
            - 127.0.0.1:8090
        - name: backend-2
          policy: Random
          health-check:
            enabled: true
          addrs:
            - 127.0.0.2:8080
            - 127.0.0.2:8090
router:
  routes:
    - id: backend-1
      matchers:
        - Proxy=http://backend-1
    - id: backend-2
      matchers:
        - Proxy=http://backend-2
```