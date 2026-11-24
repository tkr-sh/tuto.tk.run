## 更多运算符

你已经知道 `+`、`-`、`*` 和 `/`。Lua 还有另外三个用于 `number` 的运算符：

|运算符|作用|
|-|-|
|`//`|除法并去掉小数部分|
|`%`|给出除法的余数|
|`^`|求幂|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

`%` 运算符在判断一个 `number` 是偶数还是奇数时非常有用：当一个 `number` 除以 `2` 的余数为 `0` 时，它就是偶数。

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## 整数 `number` 和小数 `number`

Lua 有两种 `number`：像 `5` 这样的整数 `number`，以及像 `5.0` 或 `3.14` 这样的小数 `number`。大多数时候你不需要考虑这一点，但它解释了为什么有些结果会以 `.0` 打印：

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

`/` 运算符总是给出小数 `number`，即使结果是整数。`5` 和 `5.0` 仍然是相等的。

<div class="note">

小数 `number` 并不总是精确的！计算机无法完美存储像 `0.1` 这样的 `number`，所以有些比较会给出令人惊讶的结果：

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

避免对包含小数 `number` 的计算结果使用 `==`。

</div>

## 其他写 `number` 的方式

非常大或非常小的 `number` 可以用 `e` 来写，它的意思是“乘以 10 的多少次方”。你也可以用以 `0x` 开头的 [十六进制](https://en.wikipedia.org/wiki/Hexadecimal) 来写 `number`：

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## 转换 `number` 和字符串

`tostring` 函数把 `number` 转换成 `string`，而我们已经用过的 `tonumber` 做相反的事情：

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua 在可以的时候也会自己进行转换：看起来像 `number` 的 `string` 会在计算中变成 `number`，而 `number` 会通过 `..` 变成 `string`。

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` 和 `math.ceil`
`math.floor` 函数会把 `number` 向下取整，`math.ceil` 会向上取整：

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

要四舍五入到最接近的整数 `number`，请在使用 `math.floor` 之前加上 `0.5`：

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` 和 `math.min`
`math.max` 函数给出你传给它的 `number` 中最大的一个，`math.min` 给出最小的一个：

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
`math.abs` 函数会去掉 `number` 的符号。这在获取两个 `number` 之间的距离时很有用：

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
`math.sqrt` 函数给出一个 `number` 的平方根：

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

你可以在这里查看所有 `math` 函数：<https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
