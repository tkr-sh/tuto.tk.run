到目前为止，我们只用这种语法声明过 Lua 变量：

```lua
my_variable = "value"
```

而且它显然有效，正如我们看到的那样。但是，它可能并不总是最好的做法。
让我们看看这个场景：

<div class="runner">

```lua
function my_function(a)
    b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```
</div>

`b` 仍然在函数 _外部_ 被定义。这也许正是你想要的。但大多数时候，这并不是预期的行为。大多数时候，你希望创建一个只存在于函数中、循环中或某个特定作用域中的变量。

要让一个值只存在于特定作用域中，我们可以简单地在变量名前面加上 `local` 关键字，像这样：

```lua
local my_variable = "value"
```

这样，这个变量会一直存在，直到到达定义它的代码块（`end`、`repeat ... until` 等）的结尾；如果它是在代码块外部定义的，则直到文件结尾。这样我们就可以修复我们的例子：

<div class="runner">

```lua
function my_function(a)
    local b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```

</div>

<div class="note">

`for` 的值默认是 `local`

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

声明变量和给变量赋值是两件不同的事情：

- **声明**会创建具有 `local` 作用域的变量：`local my_variable`
- **赋值**会给变量一个值：`my_variable = "value"`

写 `local my_variable = "value"` 只是把这两件事一次完成。

你也可以声明一个变量而不给它赋值。在你给它赋值之前，它会包含 `nil`：

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

每个变量只写一次 `local`：第二次写 `local user_guess = 42` 会创建一个新变量（可能在一个新作用域中），而不是修改第一个变量。

</div>

我们可以用下面这个最终例子来完成我们的游戏：

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    local user_input

    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        local is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

local scores = {}

repeat 
	local number_of_tries = 0
	local number_to_guess = math.random(1, 100)

	print("试着猜一个 1 到 100 之间的随机数！")

	repeat
        local user_guess = validated_read(
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

    scores[#scores + 1] = number_of_tries

    local user_answer = validated_read(
        "你想继续吗？yes / no",
        "这不是一个有效的回答。",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("成绩：")

for game_number, score in ipairs(scores) do
    print("第 " .. game_number .. " 局：用了 " .. score .. " 次尝试！")
end

print("再见！")
```

</div>
