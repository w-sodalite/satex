# RateLimit

> 使用令牌桶的来限制请求的速率

## 参数

| 参数       |   类型   | 是否必须 |  默认值  | 描述                                                    |
|:---------|:------:|:----:|:-----:|:------------------------------------------------------|
| policy   | `str`  | `Y`  |       | 超过限制后的策略：`Await`：超过限制后等待直到重新填充。`Break`：超过限制后直接中断请求失败。 |
| max      | `u32`  | `Y`  |       | 最大Token数                                              |
| refill   | `u32`  | `Y`  |       | 重新填充的Token数                                           |
| interval | `u32`  | `N`  |   1   | 填充Token间隔时间，单位：秒。                                     |
| fair     | `bool` | `N`  | false | 是否使用公平模式                                              |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: rate-limit-shortcut
          layers:
            - RateLimit=Await,10,10,1,false
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: rate-limit-complete
          layers:
            - kind: RateLimit
              args:
                policy: Await
                max: 10
                refill: 10
                interval: 1
                fair: false
          service: Static=examples/resources
    ```