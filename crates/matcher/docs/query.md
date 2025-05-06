# Query

Query 是一个查询参数匹配组件，用于根据请求的查询参数（即 URL 中的参数）来匹配特定条件。

## 配置

| 参数名   | 默认值 | 描述                                |                        
|-------|-----|-----------------------------------|
| name  |     | 需要匹配的查询参数名称                       |
| value |     | 需要匹配的查询参数值 ([表达式](expression.md)) |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: query-full
      matchers:
        - kind: Query
          args:
            name: admin
            value: Equal(admin)
```

- **快捷配置**

```yaml
router:
  routes:
    - id: query-shortcut
      matchers:
        - Query=name,Equals(admin)
```