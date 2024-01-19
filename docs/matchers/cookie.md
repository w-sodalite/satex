# Cookie

> 根据请求`Cookie`中的值来匹配是否需要使用此路由

## 参数

| 参数     | 类型          | 是否必须 | 默认值 | 描述       |
|:-------|-------------|:----:|:---:|:---------|
| name   | `str`       | `Y`  |     | Cookie名称 |
| values | `[pattern]` | `Y`  |     | Cookie值  |

备注：[`pattern`类型说明](./pattern.md)

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: cookie-shortcut
          matchers:
            - Cookie=k1,Exact,v1
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: cookie-complete
          matchers:
            - kind: Cookie
              args:
                name: k1
                values:
                  - mode: Exact
                    value: v1
                    sensitive: false
          service: Static=examples/resources
    ```
