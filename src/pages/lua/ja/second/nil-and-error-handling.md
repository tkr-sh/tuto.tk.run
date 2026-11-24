最初に気づくかもしれませんが、現在のゲームは有効な数値ではない値を入力すると *クラッシュ* します。

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("もっと小さい数です！")
end
```

</div>

数値を入力しないと、次のようになります。

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

しかし、これはどういう意味でしょうか？

ここでの中心となる概念は `nil` です。Lua の型を紹介したとき、`nil` 型について簡単に見ました。

> `nil`：空。`nil` の目的は、値がないことを表すことです。

<div style="height: 3px"></div>

そして、これは _まさに_ `tonumber` が行っていることです。

- `io.read()` が `"3.14"` や `"42"` のような数値を表す文字列なら、Lua はそれを `string` 型から `number` 型に変換する方法を知っています。
- そうでなく、`io.read()` が `number` に見えない文字列なら、`tonumber` は「これを数値に変換する方法がわからないので、`nil` を返そう」と言うだけです。

したがって、`io.read()` で正しい数値表現が得られなければ、単に `nil` になります。

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## でも、なぜエラーになるの？

良い質問です！ 今のところ、`user_guess` は単に `nil` です。エラーは、この行で `<` 演算子を使って `number` と `nil` を比較しようとしたときに発生します。

```lua
if 10 < user_guess then
```

そしてエラーメッセージにはよく書かれています。

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` はエラーが発生した行を意味し、この場合は比較している行です。
- `attempt to compare number with nil` はエラーが発生した理由です。

これは実に理にかなっています！ 大小の比較という観点で、「何もないもの」と数値を比較するとはどういう意味でしょうか？ `false` なのでしょうか？ `true` なのでしょうか？ いいえ、その問い自体が意味をなしません。だから Lua は単にエラーを出すのです。

## エラーの処理
これを処理する最も簡単な方法は、`user_guess` が `nil` かどうかを確認することです。これは `==` 演算子でできます。

```lua
if user_guess == nil then
	print("これは有効な数値ではありません")
end
```

しかしこの場合、何かを表示してループを続けるだけではいけません。次の行のせいで、それが 1 回の推測として数えられてしまうからです。

```lua
number_of_tries = number_of_tries + 1
```

簡単な方法は、`~=` 演算子を使って `user_guess` が `nil` でなくなるまで、つまり有効な `number` になるまで尋ね続けることです。

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

さらに、`nil` の場合にエラーメッセージを追加することもできます。

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("これは有効な数値ではありません！")
	end
until user_guess ~= nil
```

これで次の完全なプログラムが得られます。


<div class="runner">

```lua
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
```

</div>
