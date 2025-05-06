# Before

Before 匹配组件，用于判断请求是否在指定时间之前。

## 配置

| 参数名       | 默认值 | 描述                                     |
|-----------|-----|----------------------------------------|
| date_time |     | 需要匹配的开始时间，时间格式: `yyyy-MM-dd HH:mm:ss`。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: before-full
      matchers:
        - kind: Before
          args:
            date_time: 2025-01-01 00:00:00
```

- **快捷配置**

```yaml
router:
  routes:
    - id: before-shortcut
      matchers:
        - Before=2025-01-01 00:00:00
```