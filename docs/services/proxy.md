# Proxy

> 反向代理服务，代理请求到另一个服务或地址。

## 参数

| 参数  |  类型   | 是否必须 | 默认值 | 描述     |
|:----|:-----:|:----:|:---:|:-------|
| uri | `str` | `Y`  |     | 后端服务地址 |

## 配置示例

- ### 简单模式

    ```yaml
    router:
      routes:
        - id: proxy-shortcut
          service: Proxy=https://www.satex.com
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: proxy-complete
          service: 
            kind: Proxy
            args:
              uri: https://www.satex.com
    ```