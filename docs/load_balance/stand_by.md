# StandBy

> 备用负载策略在主服务器故障时，将请求切换到备用服务器上。这种策略可以提高系统的可用性和可靠性。

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
          load_balance: StandBy
    
    router:
      routes:
        - id: stand-by-route
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
            kind: StandBy
    
    router:
      routes:
        - id: stand-by-route
          service: Proxy=http://server1
    ```