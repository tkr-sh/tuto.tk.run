现在我们已经知道了所有这些，就可以让我们的游戏历史记录工作起来了！

要制作我们的成绩历史记录，我们可以在程序最开始时创建一个新表 `scores`，每次完成一局游戏时，把 `number_of_tries` 以 `scores[#scores + 1]` 的形式添加进去，也就是添加到最后一个元素。

游戏结束后，我们可以用 `ipairs` 遍历这个表，并打印我们所有的成绩！

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

scores = {}

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

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
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
