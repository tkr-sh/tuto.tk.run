さて、残っているのは試行回数を表示することだけです。

以前、ユーザーのデータを取得した直後に

```lua
number_of_tries = number_of_tries + 1
```

で増やしました。

しかし今度は、お祝いのメッセージと同時に、何回で当てたかを表示したいと思います。

最初のゲームが終わる前に、最後の演算子を学ぶ時です。

`..`：2 つの値を連結します。

<div class="runner">

```lua
print("あれから " .. 4 .. " 年が経ちました")
```

</div>

これを踏まえると、素敵なメッセージを作れます。

```lua
print("あなたは " .. number_of_tries .. " 回で当てました！")
```

そしてついに、これが私たちのゲームです。

---

<div class="runner">

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

</div>

---

## わあ。

すごいでしょう？

あなたはもうプログラマーになる準備ができています :)！

## これで終わり？

はい、でもいいえ。

残念ながら、私は一つのことを単純化しすぎました。通常、`read` や `random` をそのまま実行することはできません。Lua は組み込み関数をいくつかのカテゴリに分けるのが好きです。`math`（数学関数用）、`io`（入出力操作用）、`os`（オペレーティングシステムとやり取りするため）……

そのため、カテゴリに属する特定の関数を使いたいときは、`category.the_function()` のように呼び出す必要があります。

今回の場合、`random` は `math` カテゴリに、`read` は `io` カテゴリに属します。`print` と `tonumber` はグローバルなので、何も指定する必要はありません。

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("1 から 100 までのランダムな数を当ててみてください！")

repeat
	user_guess = tonumber(io.read())
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

## おめでとうございます、最初の Lua プログラムを完成させました！ 🎉

基礎ができたので、現在のゲームを改良して新しい機能を追加し、その過程で新しいことを学んでいきましょう！
