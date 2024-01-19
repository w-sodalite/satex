# Echo

> 用于返回静态文件的服务

## 参数

| 参数   |  类型   | 是否必须 | 默认值 | 描述     |
|:-----|:-----:|:----:|:---:|:-------|
| path | `str` | `Y`  |     | 静态资源路径 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: proxy-shortcut
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: proxy-complete
          service: 
            kind: Static
            args:
              path: examples/resources
    ```