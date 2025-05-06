# RemoveRequestHeader

请求头中间件，用于在请求到达服务之前移除指定的请求头。

## 配置

| 参数名  | 默认值 | 描述          |
|------|-----|-------------|
| name |     | 需要移除的请求头名称。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: remove-request-header-full
      layers:
        - kind: RemoveRequestHeader
          args:
            name: x-header1
```

- **快捷模式**

```yaml
router:
  routes:
    - id: remove-request-header-shortcut
      layers:
        - RemoveRequestHeader=x-header1
```