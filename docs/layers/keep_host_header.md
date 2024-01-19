# KeepHostHeader

> 通用用在反向代理时，转发到后端的请求仍然保持客户端传递的`Host`信息。

## 参数

无

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: cors-shortcut
          layers:
            - KeepHostHeader
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: rewrite-path-complete
          layers:
            - kind: KeepHostHeader
          service: Static=examples/resources
    ```