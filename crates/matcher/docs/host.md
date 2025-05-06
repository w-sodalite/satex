# Host

Host 匹配组件，用于根据请求头中的Host来匹配请求。

## 配置

| 参数名   | 默认值 | 描述                               |
|-------|-----|----------------------------------|
| value |     | 需要匹配的Host。([表达式](expression.md)) |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: host-full
      matchers:
        - kind: Host
          args:
            value: Equal(127.0.0.1)
```

- **快捷配置**

```yaml
router:
  routes:
    - id: host-shortcut
      matchers:
        - Host=Equal(127.0.0.1)
```