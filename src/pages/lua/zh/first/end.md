那么，现在剩下的唯一事情就是打印尝试次数。

之前我们在获取用户数据之后立刻用

```lua
number_of_tries = number_of_tries + 1
```

来增加它。

但现在，我们想打印一条祝贺消息，同时告诉用户用了多少次尝试？

是时候学习第一个游戏结束之前的最后一个运算符了：

`..`：连接 2 个值。

<div class="runner">

```lua
print("已经过去 " .. 4 .. " 年了")
```

</div>

记住这一点，我们就可以创建一条漂亮的消息：

```lua
print("你在 " .. number_of_tries .. " 次尝试中猜对了！")
```

终于，这就是我们的游戏：

---

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("试着猜一个 1 到 100 之间的随机数！")

repeat
	user_guess = tonumber(read())
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

---

## 哇。

哇！是不是很厉害？

你现在已经准备好成为一名程序员了 :)！

## 结束了吗？

是也不是。

遗憾的是，我简化了一件事：通常你不能这样直接执行 `read` 和 `random`。Lua 喜欢把内置函数分到一些类别里：`math`（用于数学函数）、`io`（用于输入/输出交互）、`os`（用于与操作系统交互）……

因此，当你想使用属于某个类别的特定函数时，你需要像 `category.the_function()` 这样调用它们。

在我们的例子中，`random` 属于 `math` 类别，`read` 属于 `io` 类别。`print` 和 `tonumber` 是全局的，所以不需要指定任何东西

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("试着猜一个 1 到 100 之间的随机数！")

repeat
	user_guess = tonumber(io.read())
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

## 恭喜你，你现在完成了你的第一个 Lua 程序！🎉

现在你已经有了基础，我们将尝试改进当前的游戏，添加新功能，并在这个过程中学习新东西！
