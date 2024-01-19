# SetResponseHeader

> 为响应添加特定的响应头信息

## 参数

| 参数    |  类型   | 是否必须 |   默认值    | 描述                                         |
|:------|:-----:|:----:|:--------:|:-------------------------------------------|
| name  | `str` | `Y`  |          | 请求头名称                                      |
| value | `str` | `Y`  |          | 请求头值                                       |
| mode  | `str` | `N`  | Override | 设置模式，支持：`Append`、`IfNotPresent`、`Override` |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: set-response-header-shortcut
          layers:
            - SetResponseHeader=k1,v1,IfNotPresent
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: set-response-header-complete
          layers:
            - kind: SetResponseHeader
              args:
                name: k1
                value: v1
                mode: IfNotPresent
          service: Static=examples/resources
    ```