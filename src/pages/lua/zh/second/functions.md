你可能已经注意到，我们在验证输入时两次做了相似的代码块：

```lua
repeat
    print("试着猜一个数字：")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("这不是一个有效的数字！")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("你想继续吗？yes / no")
    user_answer = string.lower(io.read())
    if user_answer ~= "yes" and user_answer ~= "no" then
        print("这不是一个有效的回答。")
    end
until user_answer == "yes" or user_answer == "no"
```

这里有一个明显的重复模式，我会尝试用伪代码/无效语法来说明，并用 `<` 和 `>` 标出可变部分。

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## 函数的基础

当你编写程序时，通常有一条规则要遵循，那就是以模块化的方式编写代码，这样它在未来会有用，而且足够通用，能够处理大多数情况，而函数正是帮助我们实现这一点的工具。

函数基本上就是程序要执行的一组指令。函数可以接收参数，也可以返回一个值。例如下面这个函数：

```lua
function double(my_number)
    return my_number * 2
end
```

这个函数的名字是 `double`，它接收一个名为 `my_number` 的参数。这个函数的结果，也就是它返回的值，是一个单一的值：`my_number * 2`。你可以通过指定函数名，然后在括号中用 `,` 分隔参数来调用函数，例如 `my_function(value1, value2)`。在我们的例子中：

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## 回到我们的游戏！

那么，让我们优化输入处理吧！我们首先应该看看我们的函数依赖什么？这对于知道它需要哪些参数是必要的。在我们的例子中：
* 提示消息，告诉用户输入值的消息
* 错误消息，当输入无效时显示的消息
* 转换，我们对从用户那里读取的字符串应用的一种转换（`tonumber`、`string.lower`、`string.upper` 等）
* 条件，用于检查在应用转换后输入是否符合我们的期望。

`prompt_message` 和 `error_message` 比较简单，它们只是字符串。<br/>

`transform` 和 `condition` 稍微复杂一些。<br>
`transform` 接收用户输入，并返回它的修改版本。这显然是一个函数的场景：接收某些输入并返回某些东西。<br/>
`condition` 也非常相似：它接收 `transform` 的结果作为参数，并根据结果返回一个布尔值（`true`/`false`）。这显然又是一个需要 `function` 的场景。

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end
```

注意 `not`！`not` 关键字会翻转布尔值：`true` => `false`，`false` => `true`。

在我们的例子中，我们可以这样写来获取用户输入的数字：

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "试着猜一个数字：",
    "这不是一个有效的数字",
    tonumber,
    is_not_nil
)
```

这会产生与下面相同的结果：

```lua
repeat
    print("试着猜一个数字：")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("这不是一个有效的数字！")
    end
until is_valid
```

## 内联函数

注意，我们定义 `is_not_nil` 只是为了使用一次。当我们不需要在别处使用某个函数时，Lua 允许我们直接去掉名字，而不必声明一个带名字的函数，像这样：

```lua
local double = function(number)
    return number * 2
end
```

在我们的例子中，我们可以用它来这样做：

```lua
validated_read(
    "试着猜一个数字：",
    "这不是一个有效的数字",
    tonumber,
    function(nb) return nb ~= nil end
)
```

这样我们就得到了最终程序：

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("试着猜一个 1 到 100 之间的随机数！")

	repeat
        user_guess = validated_read(
            "输入一个 1 到 100 之间的数字：",
            "这不是一个有效的数字",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("这个数字更小！")
		end

		if number_to_guess > user_guess then
			print("这个数字更大！")
		end
	until user_guess == number_to_guess

	print("你在 " .. number_of_tries .. " 次尝试中猜对了！")

    user_answer = validated_read(
        "你想继续吗？yes / no",
        "这不是一个有效的回答。",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("再见！")
```

</div>
