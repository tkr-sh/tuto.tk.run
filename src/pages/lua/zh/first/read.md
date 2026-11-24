```lua
user_guess = tonumber(read())
```

好，这段代码可能看起来很奇怪（函数里面套函数？），但可以这样理解：

```lua
read_value = read()
user_guess = tonumber(read_value)
```

发生的事情如下：
1. Lua 想要计算出右边的值
2. Lua 想要执行 `tonumber`，但为此，它需要它的所有输入都有值
3. 因此，当它看到 `read()` 时，它会立刻尝试运行它，好让这个函数产生值
4. 在 `read()` 运行并返回一个值之后，`tonumber` 现在就可以执行了！

### 好，但 `read()` 在做什么？

`read()` 向用户请求一个字符串，并捕获它。

**在运行示例之前：** 不要禁用这个网站的弹窗，否则你就无法再次 `read()` 了！



<div class="runner">

```lua
print(read())
```

</div>

### 那 `tonumber` 呢？

因为 `read()` 从用户那里得到的是一个 `string`，我们不能把它和我们的随机数字比较，因为你不能把一个可能包含非数字字符的 `string` 和一个 `number` 比较（例如 `"x4%$A" > 3` 没有意义）

因此，我们需要把用户的输入转换成数字，而这正是 `tonumber` 在做的事情！

如果它的输入不是有效数字，这个函数会返回 `nil`。

<div class="runner">

```lua
print(tonumber("不是数字") == nil)
```

</div>

而且你不能把 `nil` + `number` 相加，所以请输入一个有效的数字！

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

它会从用户那里读取 `string`，把它转换成 `number`，并给这个数字加一；如果它不是有效数字，`tonumber(read())` 就会是 `nil` 值。
