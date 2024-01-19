# IpHash

> 随机负载策略随机选择一台服务器将请求发送过去。这种策略简单且易于实现，适用于没有特殊需求的情况。

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
          load_balance: Random
    
    router:
      routes:
        - id: random-route
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
            kind: Random
    
    router:
      routes:
        - id: random-route
          service: Proxy=http://server1
    ```