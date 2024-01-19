# RemoteAddr

> 根据发送请求客户端的地址来匹配是否需要使用此路由

## 参数

| 参数      | 类型      | 是否必须 | 默认值 | 描述                     |
|:--------|---------|:----:|:---:|:-----------------------|
| sources | `[str]` | `Y`  |     | 客户端源地址，支持配置CIDR。       |
| policy  | `bool`  | `Y`  |     | 处理策略：`Accept`、`Reject` |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: remote-addr-shortcut
          matchers:
            - RemoteAddr=127.0.0.1,127.0.1.1/24,Accept
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: remote-addr-complete
          matchers:
            - kind: RemoteAddr
              args:
                sources:
                  - 127.0.0.1
                  - 127.0.1.1/24
                policy: Accept
          service: Static=examples/resources
    ```
