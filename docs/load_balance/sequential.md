# Sequential

> 顺序负载策略按照服务器列表的顺序依次将请求发送过去。这种策略适用于服务器性能基本一致的情况。

## 参数

无

## 配置示例

- **简单模式**

## 配置示例

- ### 简单模式

    ```yaml
    server:
      port: 9000
    
    discovery:
      - kind: Builtin
        args:
          server: server1
          uris:
            - 127.0.0.1:9000
            - 127.0.0.1:9001
            - 127.0.0.1:9002
          interval: 10
          load_balance: Sequential
    
    router:
      routes:
        - id: sequential-route
          service: Proxy=http://server1
    ```


- ### 完整模式

    ```yaml
    server:
      port: 9000
    
    discovery:
      - kind: Builtin
        args:
          server: server1
          uris:
            - 127.0.0.1:9000
            - 127.0.0.1:9001
            - 127.0.0.1:9002
          interval: 10
          load_balance: 
            kind: Sequential
    
    router:
      routes:
        - id: sequential-route
          service: Proxy=http://server1
    ```