# Header

Header 匹配组件，用于根据请求头中的Header来匹配请求。

## 配置

| 参数名   | 默认值 | 描述                                  |
|-------|-----|-------------------------------------|
| name  |     | 需要匹配的Header名称                       |
| value |     | 需要匹配的Header值 ([表达式](expression.md)) |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: header-full
      matchers:
        - kind: Header
          args:
            name: name
            value: Equal(admin)
```

- **快捷配置**

```yaml
router:
  routes:
    - id: header-shortcut
      matchers:
        - Header=Equal(admin)
```