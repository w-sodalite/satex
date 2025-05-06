# Path

Path 匹配组件，用于根据请求的路径（即 URL 中的路径部分）来匹配特定条件。

## 配置

| 参数名      | 默认值 | 描述       |                        
|----------|-----|----------|
| patterns |     | 需要匹配的路径。 |

> 匹配的语法见[matchit](https://docs.rs/matchit/0.8.6/matchit/)文档

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: path-full
      matchers:
        - kind: Path
          args:
            patterns: /a/*ignore
```

- **快捷配置**

```yaml
router:
  routes:
    - id: path-shortcut
      matchers:
        - Path=/a/*ignore
```