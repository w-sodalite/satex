`Pattern`是一个通用的匹配规则描述，支持多种模式的匹配规则使用相同的方式进行描述。

### 结构定义

| 参数        | 类型     | 描述                  |
|:----------|--------|:--------------------|
| mode      | `str`  | 匹配模式                |
| value     | `str`  | 匹配值，部分模式不需要该字段。     |
| sensitive | `bool` | 是否大小写敏感，部分模式不需要该字段。 |

### 支持的模式：

| 匹配逻辑  | 模式(`mode`)     | 是否支持值(`value`) | 支持支持大小写敏感(`sensitive`) |
|:------|:---------------|:--------------:|:----------------------:|
| 精确匹配  | `Exact`        |      `Y`       |          `Y`           |
| 前缀匹配  | `StartsWith`   |      `Y`       |          `Y`           |
| 后缀匹配  | `EndsWith`     |      `Y`       |          `Y`           |
| 是否包含  | `Contains`     |      `Y`       |          `Y`           |
| 是否不包含 | `NotContaions` |      `Y`       |          `Y`           |
| 是否存在  | `Exists`       |      `N`       |          `N`           |
| 是否不存在 | `NotExists`    |      `N`       |          `N`           |
| 正则表达式 | `Regex`        |      `Y`       |          `N`           |

### 示例配置

- **简单配置**

    ```yaml
    
    router:
      routes:
        - id: pattern-example
          matchers:
            - Header=k1,Exact,v1,false
    ```

- **完整配置**

    ```yaml
    
    router:
      routes:
        - id: pattern-example
          matchers:
            - kind: Header
              args:
                name: k1
                values:
                  - mode: Exact
                    value: v1
                    sensitive: true
    ```

