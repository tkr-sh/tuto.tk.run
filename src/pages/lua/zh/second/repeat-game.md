现在，如果我们能在一次执行中玩多局游戏，并让用户决定什么时候停止，会怎么样？

要让游戏可以重复进行，我们只需把程序包在另一个 `repeat ... until` 循环里，像这样：

```lua
repeat
	...
	print("你在 ... 次尝试中猜对了！")

	repeat 
		print("你想继续吗？yes 或 no")
		user_answer = io.read()
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("再见！")
```

我们使用一个特殊关键字 `or`，它组合两个条件，只要其中至少一个为真，它就会是 `true`。

例如，假设我们有一个布尔值 `a`，它可以是 `true` 或 `false`，还有另一个布尔值 `b`，它也可以是 `true` 或 `false`。那么表达式 `a or b` 将会是：

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

只有当 `a` 和 `b` 都是 `false` 时，`a or b` 才是 `false`。
这与 `and` 运算符相反，`and` 要求 `a` 和 `b` 都为 `true` 时结果才是 `true`

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## 大小写

在 Lua 中，像 `"A"`、`"B"`、`"C"` 这样带大写字母的字符串与 `"a"`、`"b"`、`"c"` 并不相同。这意味着如果用户输入 `"Yes"`，它不会起作用，即使用户输入的是 `"Yes"`，也会重新执行 `repeat` 循环。这很可能不是我们想要的……

幸运的是，Lua 有一些函数可以让我们把字符串转换成全大写或全小写：`string.upper` 和 `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

这样我们就可以做下面的事情

<div class="runner">

```lua
repeat 
	print("你想继续吗？yes / no")
	user_answer = string.lower(io.read())
	if user_answer ~= "yes" and user_answer ~= "no" then
		print("这不是一个有效的回答。")
	end
until user_answer == "yes" or user_answer == "no"
```

</div>

这样，由于 `string.lower()`，`user_answer` 将始终是小写。
我们还添加了一个检查，只是为了在回答既不是 `"yes"` 也不是 `"no"` 时打印一条消息。
这样我们就得到了这个完整程序：

<div class="runner">

```lua
repeat 
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

	repeat 
		print("你想继续吗？yes / no")
		user_answer = string.lower(io.read())
		if user_answer ~= "yes" and user_answer ~= "no" then
			print("这不是一个有效的回答。")
		end
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("再见！")
```

</div>
