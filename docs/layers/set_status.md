# SetStatus

> 设置响应的状态码

## 参数

| 参数     |  类型   | 是否必须 | 默认值 | 描述  |
|:-------|:-----:|:----:|:---:|:----|
| status | `u16` | `Y`  |     | 状态码 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: set-status-shortcut
          layers:
            - SetStatus=404
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: request-body-limit-complete
          layers:
            - kind: SetStatus
              args:
                status: 404
          service: Static=examples/resources
    ```