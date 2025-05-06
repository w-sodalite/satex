# RemoteAddr

远程地址匹配组件，用于根据请求的远程地址（即客户端的 IP 地址）来匹配特定条件。

## 配置

| 参数名    | 默认值 | 描述                        |                        
|--------|-----|---------------------------|
| addrs  |     | 需要匹配的远程地址。                | 
| policy |     | 匹配处理策略: `Accept` `Reject` | 

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: remote-addr-full
      matchers:
        - kind: RemoteAddr
          args:
            policy: Accept
            addrs:
              - 192.168.0.1
              - 192.168.0.1/32
```

- **快捷配置**

```yaml
router:
  routes:
    - id: remote-addr-shortcut
      matchers:
        - RemoteAddr=Accept,192.168.0.1,192.168.0.1/32
```