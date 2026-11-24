これですべてがわかったので、ゲームの履歴機能を動かせます！

スコアの履歴を作るには、プログラムの最初に新しいテーブル `scores` を作り、ゲームを終えるたびに `number_of_tries` を `scores[#scores + 1]`、つまり最後の要素として追加します。

ゲームが終わったら、`ipairs` でテーブルを反復処理し、すべてのスコアを表示できます！

これで最終的なプログラムが得られます。

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

	print("1 から 100 までのランダムな数を当ててみてください！")

	repeat
        user_guess = validated_read(
            "1 から 100 までの数値を入力してください：",
            "これは有効な数値ではありません",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("もっと小さい数です！")
		end

		if number_to_guess > user_guess then
			print("もっと大きい数です！")
		end
	until user_guess == number_to_guess

	print("あなたは " .. number_of_tries .. " 回で当てました！")

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
        "続けますか？yes / no",
        "これは有効な回答ではありません。",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("スコア：")

for game_number, score in ipairs(scores) do
    print("ゲーム " .. game_number .. "： " .. score .. " 回でクリア！")
end

print("さようなら！")
```

</div>
