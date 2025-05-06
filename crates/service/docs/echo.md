# Echo

Echo 模块用于返回指定内容，通常用于测试、开发环境。

## 配置

| 参数名  | 默认值 | 描述       |
|------|-----|----------|
| text |     | 需要返回的内容。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: echo-full
      matchers:
        - kind: Echo
          args:
            text: 'Hello World'
```

- **快捷配置**

```yaml
router:
  routes:
    - id: echo-shortcut
      matchers:
        - Echo='Hello World'
```