## 引号

字符串可以用双引号 `"..."` 或单引号 `'...'` 来写。两者给出的字符串完全相同。当你的文本包含其中一种引号时，这会很有用：

<div class="runner">

```lua
print("今天天气很好")
print('他说“你好”')
```

</div>

## 多行字符串

你不能像这样简单地在 `"..."` 字符串中间换行：

<div class="runner">

```lua
print("第 1 行
第 2 行")
```
</div>

当用 `"` 或 `'` 写字符串时，Lua 不允许这样直接换行。

不过！你可以把 `"` 或 `'` 替换成 `[[`/`]]` 来做到：

<div class="runner">

```lua
print([[第1行
第2行]])
```

</div>

## 长度

我们用在表上的 `#` 运算符也适用于字符串：

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
`string.sub` 函数允许通过指定起始和结束索引，从 `string` 的一部分创建一个新的 `string`：

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- 等同于
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
`string.rep` 函数允许把一个 `string` 重复多次：

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
顾名思义，`string.reverse` 函数会反转一个 `string`：

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
`string.gsub` 函数会把一个 `string` 替换成另一个，并且还会打印它被替换了多少次：

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

你可以在这里查看所有 `string` 函数：<https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
