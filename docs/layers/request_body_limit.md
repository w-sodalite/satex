# RequestBodyLimit

> 限制请求的包体大小

## 参数

| 参数  |  类型   | 是否必须 | 默认值 | 描述              |
|:----|:-----:|:----:|:---:|:----------------|
| max | `u32` | `Y`  |     | 允许的最大包体大小，单位：字节 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: request-body-limit-shortcut
          layers:
            - RequestBodyLimit=10240
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: request-body-limit-complete
          layers:
            - kind: RequestBodyLimit
              args:
                max: 10240
          service: Static=examples/resources
    ```