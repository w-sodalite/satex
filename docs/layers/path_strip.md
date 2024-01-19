# PathStrip

> 从请求路径中删除特定部分

## 参数

| 参数    |  类型   | 是否必须 | 默认值 | 描述    |
|:------|:-----:|:----:|:---:|:------|
| level | `u32` | `Y`  |     | 截取的级别 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: path-strip-shortcut
          layers:
            - PathStrip=1
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: rewrite-path-complete
          layers:
            - kind: PathStrip
              args:
                level: 1
          service: Static=examples/resources
    ```