# SetStatus

设置状态码中间件，用于在响应到达客户端之前设置状态码。

## 配置

| 参数名    | 默认值 | 描述        |
|--------|-----|-----------|
| status |     | 需要设置的状态码。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: set-status-full
      layers:
        - kind: SetStatus
          args:
            status: 200
```

- **快捷模式**

```yaml
router:
  routes:
    - id: set-status-shortcut
      layers:
        - SetStatus=200
```