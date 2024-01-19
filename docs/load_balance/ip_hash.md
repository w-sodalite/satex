# IpHash

> IP哈希负载策略使用客户端的IP地址进行哈希计算，根据哈希值将请求分配给后端服务器。这样可以确保来自同一IP地址的请求始终被发送到同一台服务器，这有助于保持会话和状态信息的持续性。

## 参数

| 参数       |  类型   | 是否必须 | 默认值  | 描述                   |
|:---------|:-----:|:----:|:----:|:---------------------|
| timeout  | `u32` | `N`  | 1800 | IP对应的节点缓存的超时时间，单位：秒  |
| interval | `u32` | `N`  |  10  | 清理超时的IP节点缓存间隔时间，单位：秒 |

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
          load_balance: IpHash=1800,10
    
    router:
      routes:
        - id: ip-hash-route
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
            kind: IpHash
            args:
              timeout: 1800
              interval: 10
    
    router:
      routes:
        - id: ip-hash-route
          service: Proxy=http://server1
    ```