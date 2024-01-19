# IpHash

> IP哈希负载策略使用客户端的IP地址进行哈希计算，根据哈希值将请求分配给后端服务器。这样可以确保来自同一IP地址的请求始终被发送到同一台服务器，这有助于保持会话和状态信息的持续性。

## 参数

| 参数     |   类型    | 是否必须 | 默认值 | 描述        |
|:-------|:-------:|:----:|:---:|:----------|
| ratios | `[u32]` | `Y`  |     | 所有节点的权重信息 |

备注：这里权重信息的个数应该和节点个数保持一致。

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
          load_balance: Weight=1,1,2
    
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
            kind: Weight
            args:
              ratios:
                - 1
                - 1
                - 2
    
    router:
      routes:
        - id: ip-hash-route
          service: Proxy=http://server1
    ```