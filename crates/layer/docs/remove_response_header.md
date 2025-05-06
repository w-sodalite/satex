# RemoveResponseHeader

响应头中间件，用于在响应到达客户端之前移除指定的响应头。

## 配置

| 参数名  | 默认值 | 描述          |
|------|-----|-------------|
| name |     | 需要移除的请求头名称。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: remove-response-header-full
      layers:
        - kind: RemoveResponseHeader
          args:
            name: x-header1
```

- **快捷模式**

```yaml
router:
  routes:
    - id: remove-response-header-shortcut
      layers:
        - RemoveResponseHeader=x-header1
```