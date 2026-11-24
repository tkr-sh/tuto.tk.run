## 内联函数

到目前为止，我们这样写函数：

```lua
function double(number)
    return number * 2
end
```

这是语法糖。Lua 实际做的是创建一个没有名字的函数，并把它存到一个名为 `double` 的变量中：

```lua
double = function(number)
    return number * 2
end
```

两个版本完全相同。这也是为什么我们可以把 `tonumber` 传给 `validated_read`：函数只是一个存储在变量中的值，就像数字或字符串一样。


## 用 `:` 调用函数

为了把字符串转换成小写，我们写过 `string.lower(user_answer)`。
但是，当你使用 `string` 或 `table` 库中的函数，并且它把 `string` 或 `table` 作为第一个参数时，你可以用下面的语法来写：`user_answer:lower()`：

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

`:` 会取它左边的值，并把它作为第一个参数传给函数。对于字符串，Lua 会在 `string` 表中查找这个函数，所以 `answer:lower()` 就变成了 `string.lower(answer)`。

<div class="note">

这对于直接写在代码中的字符串不起作用：`"YES":lower()` 是语法错误，因为 Lua 只接受变量名或括号后面的 `:`。要让它工作，请把字符串放在括号中：

```lua
print(("YES"):lower())
-- 这会报错：
print("YES":lower())
```

</div>

## 不带括号

当调用函数时只有一个参数，并且这个参数是直接写在代码中的 `string` 或 `table`，括号是可选的：

<div class="runner">

```lua
print "Hello!"
print("Hello!")
```

</div>

你会在别人的代码中看到这种写法。在本课程中，我们保留括号，因为它们能清楚地表明正在调用函数。

## 返回多个值

函数可以返回不止一个值。你在 `return` 后面用 `,` 分隔它们，并用多个变量接收它们，同样用 `,` 分隔：

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

如果你给的变量比值的数量少，多出来的值会被直接丢弃。如果你给的变量更多，多出来的变量就是 `nil`：

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

如果只想保留 `function` 的第一个值，请把调用放在括号中：

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## 缺少和多出的参数

Lua 不会检查你传给 `function` 的参数数量。缺少的参数是 `nil`，多余的参数会被忽略：

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## 可选参数

由于缺少的参数是 `nil`，我们可以检查它并改用默认值。这会让参数变成可选的：

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Hello"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Good evening")
```
