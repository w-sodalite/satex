# SetResponseHeader

响应头中间件，用于在响应到达客户端之前设置响应头。

## 配置

| 参数名    | 默认值          | 描述                                                |
|--------|--------------|---------------------------------------------------|
| name   |              | 需要设置的请求头名称                                        |
| value  |              | 需要设置的请求头值                                         |
| policy | `Overriding` | 设置值的策略,支持 `Overriding` `Appending` `IfNotPresent` |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: set-response-header-full
      layers:
        - kind: SetResponseHeader
          args:
            name: x-header1
            value: value1
            policy: Overriding
```

- **快捷模式**

```yaml
router:
  routes:
    - id: set-response-header-shortcut
      layers:
        - SetResponseHeader=x-header1,value1,Overriding
```