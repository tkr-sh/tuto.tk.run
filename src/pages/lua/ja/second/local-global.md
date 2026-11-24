これまで、私たちは次の構文だけで Lua の変数を宣言してきました。

```lua
my_variable = "value"
```

そして、見てきたとおり、それは明らかに機能します。しかし、それが常に最善の方法とは限りません。
次のシナリオを見てみましょう。

<div class="runner">

```lua
function my_function(a)
    b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```
</div>

`b` はまだ関数の _外側_ で定義されています。それが望ましい場合もあります。しかしほとんどの場合、これは期待される動作ではありません。ほとんどの場合、関数の中、ループの中、単一のスコープの中だけで存在する変数を作りたいはずです。

値を特定のスコープの中だけに存在させるには、次のように変数名の前に `local` キーワードを置くだけです。

```lua
local my_variable = "value"
```

これで、変数は定義されたブロック（`end`、`repeat ... until` など）の終わりまで、またはブロックの外で定義された場合はファイルの終わりまで存在します。これで例を修正できます。

<div class="runner">

```lua
function my_function(a)
    local b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```

</div>

<div class="note">

`for` の値はデフォルトで `local` です。

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

変数の宣言と、変数への値の代入は別のことです。

- **宣言**は `local` のスコープを持つ変数を作ります：`local my_variable`
- **代入**は変数に値を与えます：`my_variable = "value"`

`local my_variable = "value"` と書くと、単にこの 2 つを一度に行うだけです。

値を与えずに変数を宣言することもできます。何かを代入するまで、それは `nil` を含みます。

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

`local` は変数ごとに一度だけ書いてください。2 回目の `local user_guess = 42` は、最初の変数を変更するのではなく、（おそらく新しいスコープに）新しい変数を作ります。

</div>

そして、この最後の例でゲームを完成させられます。

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    local user_input

    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        local is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

local scores = {}

repeat 
	local number_of_tries = 0
	local number_to_guess = math.random(1, 100)

	print("1 から 100 までのランダムな数を当ててみてください！")

	repeat
        local user_guess = validated_read(
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

    local user_answer = validated_read(
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
