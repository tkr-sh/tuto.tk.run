## 什么算作 `true`？

在条件中，Lua 不只是接受 `true` 和 `false`：任何值都可以使用。规则很简单：`false` 和 `nil` 算作假，而**其他所有值**都算作真。

如果你了解其他编程语言，请注意：在 Lua 中，`0` 和空字符串 `""` 都算作真！

<div class="runner">

```lua
if 0 then
    print("0 算作真")
end

if "" then
    print("空字符串算作真")
end

if nil then
    print("这永远不会被打印")
end
```

</div>

## 不用 `== nil` 检查 `nil`

多亏这条规则，我们可以通过直接把变量用作条件来检查它是否有值：

<div class="runner">

```lua
local name = nil

if name then
    print("你好 " .. name)
else
    print("没有名字")
end
```

</div>

`if name then` 是 `if name ~= nil then` 的简短写法，而 `if not name then` 是 `if name == nil then` 的简短写法。

<div class="note">

当变量可能包含 `false` 时，这两者并不完全一样：`false` 不是 `nil`，但它在条件中也被算作假。

</div>

## `not`

`not` 关键字总是给出真正的布尔值。对于 `false` 和 `nil`，它给出 `true`；对于其他所有值，它给出 `false`：

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or` 和 `and` 会返回其中一个值

我们用布尔值看过 `or` 和 `and`，但它们可以处理任何值，而且不总是返回布尔值：它们会返回两个值中的一个。

- 如果 `a` 算作真，`a or b` 返回 `a`，否则返回 `b`。
- 如果 `a` 算作假，`a and b` 返回 `a`，否则返回 `b`。

<div class="runner">

```lua
print(nil or "默认值")
print("值" or "默认值")

print(nil and "下一个")
print("值" and "下一个")
```

</div>

对于 `true` 和 `false`，这得到的正是我们之前看到的表格。

## 用 `or` 设置默认值

这就是为什么 `or` 可以为变量提供默认值，就像我们为可选参数所做的那样：

<div class="runner">

```lua
local name = nil
name = name or "访客"

print(name)
```

</div>

如果 `name` 有值，就保留它。如果 `name` 是 `nil`，它就会变成 `"访客"`。

## 在两个值之间选择

通过把 `and` 和 `or` 一起使用，我们可以在一行中根据条件在两个值之间选择：

<div class="runner">

```lua
local score = 70
local result = score > 50 and "你赢了！" or "你输了！"

print(result)
```

</div>

这等同于：

```lua
local result
if score > 50 then
    result = "你赢了！"
else
    result = "你输了！"
end
```

<div class="note">

如果第一个值是 `false` 或 `nil`，这个技巧就不起作用：`condition and false or "其他"` 总是给出 `"其他"`。在这种情况下，请使用普通的 `if`。

</div>

## `and` 会尽可能早地停止

当 `and` 的第一个值算作假时，Lua 甚至不会看第二个值。还记得我们在课程开头比较 `nil` 和数字时遇到的错误吗？`and` 可以让我们避免它：

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("猜测值大于 10")
end

print("没有错误！")
```

</div>

因为 `user_guess ~= nil` 是 `false`，所以 `user_guess > 10` 永远不会执行。同样，当 `or` 的第一个值算作真时，它也不会看第二个值。

## 表中的 `nil`

表中不存在的键会给出 `nil`，不会报错。而要从表中删除一个键，你可以给它赋 `nil`：

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

这是删除像 `score` 这样的键的正确方式，但不适用于列表中的值：正如我们看到的，那会创建一个空洞。对于列表，请使用 `table.remove`。

</div>
