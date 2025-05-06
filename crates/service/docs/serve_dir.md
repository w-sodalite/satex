# ServeDir

ServeDir 模块用于提供静态文件服务，支持目录的递归遍历和文件缓存。

## 配置

| 参数名  | 默认值 | 描述      |
|------|-----|---------|
| path |     | 静态文件目录。 |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: serve-dir-full
      matchers:
        - kind: ServeDir
          args:
            path: /srv/html
```

- **快捷配置**

```yaml
router:
  routes:
    - id: serve-dir-shortcut
      matchers:
        - ServeDir=/srv/html
```