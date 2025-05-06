# SetRequestHeader

请求头中间件，用于在请求到达服务之前设置请求头。

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
    - id: set-request-header-full
      layers:
        - kind: SetRequestHeader
          args:
            name: x-header1
            value: value1
            policy: Overriding
```

- **快捷模式**

```yaml
router:
  routes:
    - id: set-request-header-shortcut
      layers:
        - SetRequestHeader=x-header1,value1,Overriding
```