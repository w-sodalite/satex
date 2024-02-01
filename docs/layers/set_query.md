# SetQuery

> 为请求添加Query参数

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
        - id: set-query-shortcut
          layers:
            - SetQuery=k1,v1,IfNotPresent
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: set-query-complete
          layers:
            - kind: SetQuery
              args:
                name: k1
                value: v1
                mode: IfNotPresent
          service: Static=examples/resources
    ```