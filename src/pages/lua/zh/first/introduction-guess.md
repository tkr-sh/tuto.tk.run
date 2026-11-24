这是我们将要学习的第一个程序！别担心，我们会非常非常慢地讲解它，所以即使你现在什么都不懂也没关系！

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

这个程序做的是创建“找数字”游戏：

```
试着猜一个 1 到 100 之间的随机数！
50
这个数字更大！
75
这个数字更小！
62
你在 3 次尝试中猜对了！
```
