# Cors

> 用于处理跨域请求

## 参数

| 参数                    |   类型    | 是否必须 |  默认值  | 描述         |
|:----------------------|:-------:|:----:|:-----:|:-----------|
| allow_credentials     | `bool`  | `N`  | false | 是否允许携带认证信息 |
| allow_headers         | `[str]` | `N`  |  []   | 允许携带的头     |
| allow_methods         | `[str]` | `N`  | [GET] | 允许请求的方法    |
| allow_origin          | `[str]` | `N`  |  []   | 允许的域       |
| allow_private_network | `bool`  | `N`  | false | 是否允许私有网络   |
| expose_headers        | `[str]` | `N`  |  []   | 允许返回的头     |
| max_age               |  `u32`  | `N`  |   0   | 最大有效期      |
| vary                  | `[str]` | `N`  |  []   | vary列表     |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: cors-shortcut
          layers:
            - Cors
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: rewrite-path-complete
          layers:
            - kind: Cors
              args:
                allow_credentials: false
                allow_origin: '*'
          service: Static=examples/resources
    ```