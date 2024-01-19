# ConcurrencyLimit

> 限制请求的同时并发数量

## 参数

| 参数  |  类型   | 是否必须 | 默认值 | 描述        |
|:----|:-----:|:----:|:---:|:----------|
| max | `u32` | `Y`  |     | 允许的最大并发数量 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: concurrency-limit-shortcut
          layers:
            - ConcurrencyLimit=10
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: concurrency-limit-complete
          layers:
            - kind: ConcurrencyLimit
              args:
                max: 10
          service: Static=examples/resources
    ```