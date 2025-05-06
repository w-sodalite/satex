# StripPrefix

路径前缀剥离中间件，用于自动移除请求路径中的指定层级前缀。

## 配置

| 参数名   | 默认值 | 描述                                   |
|-------|-----|--------------------------------------|
| level |     | 需要剥离的路径层级数。路径层级按 `/` 分割计算，空字符串不计入层级。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: strip-prefix-full
      layers:
        - kind: StripPrefix
          args:
            level: 1
```

- **快捷模式**

```yaml
router:
  routes:
    - id: strip-prefix-shortcut
      layers:
        - StripPrefix=1
```