これは私たちがこれから学ぶ最初のプログラムです！ 心配しないでください。ものすごくゆっくり進めますので、まだ何もわからなくても大丈夫です！

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("1 から 100 までのランダムな数を当ててみてください！")

repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("もっと小さい数です！")
	end

	if number_to_guess > user_guess then
		print("もっと大きい数です！")
	end
until user_guess == number_to_guess

print("あなたは " .. number_of_tries .. " 回で当てました！")
```

このプログラムが行うのは、「数を当てる」ゲームを作ることです。

```
1 から 100 までのランダムな数を当ててみてください！
50
もっと大きい数です！
75
もっと小さい数です！
62
あなたは 3 回で当てました！
```
