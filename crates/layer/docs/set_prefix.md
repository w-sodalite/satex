# SetPrefix

路径前缀设置中间件，用于设置请求路径的起始部分。

## 功能概述

- **核心作用**：根据配置的层级数（`level`）剥离请求路径的前缀部分。
- **典型场景**：当服务部署在 `/api/v1` 路径下时，可通过设置 `level=2` 将 `/api/v1/resource` 转换为 `/resource`。

---

## 配置

| 参数名   | 默认值 | 描述                                   |
|-------|-----|--------------------------------------|
| level |     | 需要剥离的路径层级数。路径层级按 `/` 分割计算，空字符串不计入层级。 |

---

## 示例配置

- **完整配置模式**

```yaml
router:
  routes:
    - id: set-prefix-full
      layers:
        - kind: SetPrefix
          args:
            level: 1
```

- **快捷模式**

```yaml
router:
  routes:
    - id: set-prefix-shortcut
      layers:
        - SetPrefix=1
```
