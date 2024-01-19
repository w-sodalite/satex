# Path

> 根据请求的地址来匹配是否需要使用此路由

## 参数

| 参数       | 类型      | 是否必须 | 默认值 | 描述   |
|:---------|---------|:----:|:---:|:-----|
| patterns | `[str]` | `Y`  |     | 接口地址 |

备注：这里的路径支持`path-tree`模式，[使用方式见文档](https://github.com/viz-rs/path-tree/blob/main/README.md)
。可以在路径中绑定变量，然后在`Layer - RewritePath`中使用。

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: path-shortcut
          matchers:
            - Path=/satex1/:v+,/satex2/:v+
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: path-complete
          matchers:
            - kind: Path
              args:
                patterns:
                  - /satex1/:v+
                  - /satex2/:v+
          service: Static=examples/resources
    ```