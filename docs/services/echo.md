# Echo

> 简单地返回接收到的请求内容，主要用于测试。

## 参数

无

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: echo-shortcut
          service: Echo
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: echo-complete
          service: 
            kind: Echo
    ```