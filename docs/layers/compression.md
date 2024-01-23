# Compression

> 用来压缩响应包体

## 参数

| 参数                    |   类型    | 是否必须 |  默认值  | 描述                   |
|:----------------------|:-------:|:----:|:-----:|:---------------------|
| gzip                  | `bool`  | `N`  | true  | 支持`gzip`压缩           |
| deflate               | `bool`  | `N`  | false | 支持`deflate`压缩        |
| br                    | `bool`  | `N`  | false | 支持`br`压缩             |
| zstd                  | `bool`  | `N`  | false | 支持`zstd`压缩           |
| level                 |  `i32`  | `N`  |   5   | 压缩级别，越小压缩率越低，但是速度越快。 |
| above_size            |  `u16`  | `N`  |  128  | 压缩的最小包体大小            |
| exclude_content_types | `[str]` | `N`  |       | 指定不需要压缩的content_type |

## 配置示例

- ### 简单配置

  > 不支持配置值，使用默认值。

    ```yaml
    router:
      routes:
        - id: static
          layers:
            - Compression
          service: Static=examples/resources
    ```

- ### 完整模式

    ```yaml
    router:
      routes:
        - id: static
          layers:
            - kind: Compression
              args:
                gzip: true
                deflate: true
                above_size: 128
                exclude_content_types:
                  - application/grpc
          service: Static=examples/resources
    ```