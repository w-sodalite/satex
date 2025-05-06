# Between

Between 匹配组件，用于判断请求是否在指定区间内。

## 配置

| 参数名   | 默认值 | 描述                                     |
|-------|-----|----------------------------------------|
| start |     | 需要匹配的开始时间，时间格式: `yyyy-MM-dd HH:mm:ss`。 |
| end   |     | 需要匹配的开始时间，时间格式: `yyyy-MM-dd HH:mm:ss`。 | 

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: between-full
      matchers:
        - kind: Between
          args:
            start: 2025-01-01 00:00:00
            end: 2026-01-01 00:00:00
```

- **快捷配置**

```yaml
router:
  routes:
    - id: between-shortcut
      matchers:
        - Between=2025-01-01 00:00:00,2026-01-01 00:00:00
```