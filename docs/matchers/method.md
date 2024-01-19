# Method

> 根据请求的`Method`来匹配是否需要使用此路由

## 参数

| 参数      | 类型      | 是否必须 | 默认值 | 描述  |
|:--------|---------|:----:|:---:|:----|
| methods | `[str]` | `Y`  |     | 方法名 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: method-shortcut
          matchers:
            - Method=GET,POST
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: method-complete
          matchers:
            - kind: Method
              args:
                methods:
                  - GET
                  - POST
          service: Static=examples/resources
    ```
