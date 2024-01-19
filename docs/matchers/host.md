# Host

> 根据请求`Host`中的值来匹配是否需要使用此路由

## 参数

| 参数     | 类型          | 是否必须 | 默认值 | 描述    |
|:-------|-------------|:----:|:---:|:------|
| values | `[pattern]` | `Y`  |     | Host值 |

备注：[`pattern`类型说明](./pattern.md)

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: host-shortcut
          matchers:
            - Host=Exact,www.satex.com
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: header-complete
          matchers:
            - kind: Host
              args:
                values:
                  - mode: Exact
                    value: www.satex.com
          service: Static=examples/resources
    ```
