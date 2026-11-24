你可能注意到的第一件事是，当我们输入一个不是有效数字的值时，我们的游戏目前会 *崩溃*：

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("这个数字更小！")
end
```

</div>

如果你不输入数字，它会给你：

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

但这是什么意思呢？

这里的核心概念是 `nil`。在介绍 Lua 类型时，我们简单看过 `nil` 类型：

> `nil`：空。`nil` 的作用是表示你没有值。

<div style="height: 3px"></div>

而这 _正是_ `tonumber` 在做的事情：

- 如果 `io.read()` 是一个表示数字的字符串，比如 `"3.14"` 或 `"42"`，Lua 知道如何把它从 `string` 类型转换成 `number` 类型。
- 否则，如果 `io.read()` 是一个看起来不像 `number` 的字符串，`tonumber` 就会说“我不知道如何把它转换成数字，那就返回 `nil` 吧”。

因此，如果我们在 `io.read()` 中没有得到正确的数字表示，我们就只会得到 `nil`：

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## 好，但为什么它会报错？

好问题！目前，`user_guess` 只是 `nil`。错误发生在我们尝试用 `<` 运算符把一个 `number` 和 `nil` 比较时，也就是这一行：

```lua
if 10 < user_guess then
```

而且错误消息已经描述得很清楚了：

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` 表示错误发生的行号，在这个例子中就是我们比较的那一行。
- `attempt to compare number with nil` 是发生错误的原因。

这非常合理！从大小比较的角度来说，把“什么都没有”和一个数字比较是什么意思？它是 `false` 吗？它是 `true` 吗？都不是，这个问题本身就没有意义。所以 Lua 就直接报错了。

## 处理错误
处理这个问题最快的方法就是检查 `user_guess` 是否为 `nil`，这可以用 `==` 运算符来完成：

```lua
if user_guess == nil then
	print("这不是一个有效的数字")
end
```

但在这种情况下，我们不希望只是打印一些东西然后继续循环，因为这会因为下面这行而被算作一次猜测：

```lua
number_of_tries = number_of_tries + 1
```

一个简单的方法是，用 `~=` 运算符一直询问 `user_guess`，直到它不是 `nil`，也就是说，直到它是一个有效的 `number`：

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

我们还可以添加一条错误消息，以防它是 `nil`：

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("这不是一个有效的数字！")
	end
until user_guess ~= nil
```

这样我们就得到了这个完整程序：


<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("试着猜一个 1 到 100 之间的随机数！")

repeat
	repeat
		user_guess = tonumber(io.read())
		if user_guess == nil then 
			print("这不是一个有效的数字！")
		end
	until user_guess ~= nil
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("这个数字更小！")
	end

	if number_to_guess > user_guess then
		print("这个数字更大！")
	end
until user_guess == number_to_guess

print("你在 " .. number_of_tries .. " 次尝试中猜对了！")
```

</div>
