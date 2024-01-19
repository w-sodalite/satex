# Time

> 根据当前系统时间来匹配是否需要使用此路由

## 参数

| 参数   | 类型    | 是否必须 | 默认值 | 描述                          |
|:-----|-------|:----:|:---:|:----------------------------|
| mode | `str` | `Y`  |     | 模式：`Before`、`After`         |
| time | `str` | `Y`  |     | 目标时间，格式为`%Y-%m-%d %H:%M:%S` |

备注：这里时间使用的是本地时区。

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: time-shortcut
          matchers:
            - Time=Before,2024-12-31 23:59:59
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: time-complete
          matchers:
            - kind: Time
              args:
                mode: Before
                time: 2024-12-31 23:59:59
          service: Static=examples/resources
    ```
