# Expression

`Expression` 枚举可以表示多种匹配逻辑，并支持大小写敏感/不敏感控制。

## 表达式类型

| 类型              | 说明             |
|-----------------|----------------|
| `Equals`        | 判断是否相等         |
| `NotEquals`     | 判断是否不相等        |
| `StartsWith`    | 判断是否以某字符串开头    |
| `NotStartsWith` | 判断是否不以某字符串开头   |
| `EndsWith`      | 判断是否以某字符串结尾    |
| `NotEndsWith`   | 判断是否不以某字符串结尾   |
| `Contains`      | 判断是否包含某子字符串    |
| `NotContains`   | 判断是否不包含某子字符串   |
| `Exists`        | 判断输入值是否存在（非空）  |
| `NotExists`     | 判断输入值是否不存在（为空） |
| `Regex`         | 使用正则表达式进行匹配    |

> 注意：除了`Regex` `Exists` `NotExists`, 其他的类型都支持使用`?`来进行非大小写敏感匹配。

## 示例

```yaml
router:
  routes:
    - id: route-1
      matchers:
        # 只能匹配`admin`
        - Query=name,Equals(admin)
```

```yaml
router:
  routes:
    - id: route-1
      matchers:
        # 大小写不敏感 匹配`admin`和`Admin`
        - Query=name,?Equals(admin)
```

```yaml
router:
  routes:
    - id: route-1
      matchers:
        - Query=name,NotEquals(admin)
```