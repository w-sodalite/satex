# Method

Method 匹配组件，用于根据请求的方法（即 HTTP 请求中的方法，如 GET、POST、PUT、DELETE 等）来匹配特定条件。

## 配置

| 参数名     | 默认值 | 描述       |
|---------|-----|----------|
| methods |     | 需要匹配的方法。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: method-full
      matchers:
        - kind: Method
          args:
            methods:
              - GET
              - POST
```

- **快捷配置**

```yaml
router:
  routes:
    - id: method-shortcut
      matchers:
        - Method=GET,POST
```