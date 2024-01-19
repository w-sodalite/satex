# Builtin

> 内置的服务发现，通过配置的方式注册服务集合，内部会定时检测服务节点的健康状态。

## 参数

| 参数           |   类型    | 是否必须 | 默认值 | 描述                  |
|:-------------|:-------:|:----:|:---:|:--------------------|
| server       |  `str`  | `Y`  |     | 服务名称                |
| uris         | `[str]` | `Y`  |     | 服务节点列表              |
| interval     |  `u32`  | `Y`  |     | 服务节点健康状态检查间隔时间，单位：秒 |
| load_balance |  `str`  | `Y`  |     | 负载均衡策略              |

## 配置示例

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
    - id: discovery-route
      service: Proxy=http://server1
```