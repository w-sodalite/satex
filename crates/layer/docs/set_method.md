# SetMethod

设置请求方法中间件，用于在请求到达服务之前设置请求方法。

## 配置

| 参数名    | 默认值 | 描述         |
|--------|-----|------------|
| method |     | 需要设置的请求方法。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: set-method-full
      layers:
        - kind: SetMethod
          args:
            method: POST
```

- **快捷模式**

```yaml
router:
  routes:
    - id: set-method-shortcut
      layers:
        - SetMethod=POST
```