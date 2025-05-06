# ConcurrencyLimit

并发限制中间件，用于在请求到达服务之前设置并发限制。

## 配置

| 参数名 | 默认值 | 描述        |
|-----|-----|-----------|
| max |     | 允许的最大并发数。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: concurrency-limit-full
      layers:
        - kind: ConcurrencyLimit
          args:
            max: 10
```

- **快捷配置**

```yaml
router:
  routes:
    - id: concurrency-limit-shortcut
      layers:
        - ConcurrencyLimit=10
```