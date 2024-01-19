# RewritePath

> 用于重写请求接口的URL地址，支持使用在[`Matcher - Path`](../matchers/path.md)中绑定的变量。

## 参数

| 参数   |   类型    | 是否必须 | 默认值 | 描述    |
|:-----|:-------:|:----:|:---:|:------|
| path | `[str]` | `Y`  |     | 重写的路径 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: rewrite-path-shortcut
          matchers:
            - Path=/a/:x+
          layers:
            - RewritePath=/b/{x}
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: rewrite-path-complete
          matchers:
            - Path=/a/:x+
          layers:
            - kind: RewritePath
              args:
                path: /b/{x}
          service: Static=examples/resources
    ```