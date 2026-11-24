一度の実行で複数回ゲームをプレイでき、ユーザーがいつ止めるかを決められるとしたらどうでしょう？

ゲームを繰り返し遊べるようにするには、次のようにプログラム全体をもう 1 つの `repeat ... until` ループで包むだけです。

```lua
repeat
	...
	print("あなたは ... 回で当てました！")

	repeat 
		print("続けますか？yes または no")
		user_answer = io.read()
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("さようなら！")
```

ここでは特別なキーワード `or` を使っています。これは 2 つの条件を組み合わせ、少なくとも一方が真なら `true` になります。

たとえば、`true` か `false` を取るブール値 `a` と、同じく `true` か `false` を取る別のブール値 `b` があるとします。すると式 `a or b` は次のようになります。

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` が `false` になるのは、`a` と `b` の両方が `false` のときだけです。
これは `and` 演算子の逆で、`and` は結果が `true` になるために `a` と `b` の両方が `true` であることを要求します。

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## 大文字と小文字

Lua では、`"A"`、`"B"`、`"C"` のように大文字を含む文字列は、`"a"`、`"b"`、`"c"` とは異なります。つまり、ユーザーが `"Yes"` と入力しても動作せず、ユーザーが `"Yes"` と入力したにもかかわらず、`repeat` ループをやり直すことになります。これはおそらく望ましい動作ではありません……

幸いなことに、Lua には文字列をすべて大文字またはすべて小文字に変換する関数があります。`string.upper` と `string.lower` です。

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

これにより次のことができます。

<div class="runner">

```lua
repeat 
	print("続けますか？yes / no")
	user_answer = string.lower(io.read())
	if user_answer ~= "yes" and user_answer ~= "no" then
		print("これは有効な回答ではありません。")
	end
until user_answer == "yes" or user_answer == "no"
```

</div>

これで、`string.lower()` によって `user_answer` は常に小文字になります。
また、回答が `"yes"` でも `"no"` でもない場合にメッセージを表示するチェックも追加しました。
これで次の完全なプログラムが得られます。

<div class="runner">

```lua
repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("1 から 100 までのランダムな数を当ててみてください！")

	repeat
		repeat
			user_guess = tonumber(io.read())
			if user_guess == nil then 
				print("これは有効な数値ではありません！")
			end
		until user_guess ~= nil
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("もっと小さい数です！")
		end

		if number_to_guess > user_guess then
			print("もっと大きい数です！")
		end
	until user_guess == number_to_guess

	print("あなたは " .. number_of_tries .. " 回で当てました！")

	repeat 
		print("続けますか？yes / no")
		user_answer = string.lower(io.read())
		if user_answer ~= "yes" and user_answer ~= "no" then
			print("これは有効な回答ではありません。")
		end
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("さようなら！")
```

</div>
