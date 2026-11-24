入力の検証のために、似たようなコードブロックを 2 回書いていることに気づいたかもしれません。

```lua
repeat
    print("数を当ててみてください：")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("これは有効な数値ではありません！")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("続けますか？yes / no")
    user_answer = string.lower(io.read())
    if user_answer ~= "yes" and user_answer ~= "no" then
        print("これは有効な回答ではありません。")
    end
until user_answer == "yes" or user_answer == "no"
```

明らかな共通の重複パターンがあります。これを `<` と `>` で囲んだ疑似コード／無効な構文で示してみます。

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## 関数の基本

プログラムを書くとき、よく守るべきルールがあります。それは、将来役に立ち、ほとんどのケースを扱える十分に汎用的な形で、モジュール化してコードを書くことです。関数はまさにそれを実現する助けになります。

関数は基本的に、プログラムが実行する命令の集まりです。関数は引数を受け取り、値を返すことができます。たとえば次の関数を見てください。

```lua
function double(my_number)
    return my_number * 2
end
```

この関数の名前は `double` で、`my_number` という 1 つの引数を受け取ります。この関数の結果、つまり返す値は `my_number * 2` という 1 つの値です。関数を呼び出すには、名前を指定し、括弧の中に `,` で区切って引数を書きます。たとえば `my_function(value1, value2)` のようになります。今回の場合：

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## ゲームに戻りましょう！

では、入力処理を最適化しましょう！ まず見るべきは、関数が何に依存しているかです。これはどの引数が必要かを知るために必要です。今回の場合：
* プロンプトメッセージ、ユーザーに値を入力するよう伝えるメッセージ
* エラーメッセージ、入力が有効でないときに表示するメッセージ
* 変換、ユーザーから読み取った文字列に適用する変換（`tonumber`、`string.lower`、`string.upper` など）
* 条件、変換を適用した後に入力が期待どおりか確認するためのチェック。

`prompt_message` と `error_message` は簡単で、単なる文字列です。<br/>

`transform` と `condition` はもう少し複雑です。<br>
`transform` はユーザー入力を受け取り、変更されたバージョンを返します。これは明らかに関数のケースです。何かを入力として受け取り、何かを返すものです。<br/>
`condition` もよく似ています。引数として `transform` の結果を受け取り、その結果に応じてブール値（`true`/`false`）を返します。これも明らかに `function` が必要なケースです。

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
```

`not` に注目してください！ `not` キーワードはブール値を反転します。`true` => `false`、`false` => `true` です。

今回の場合、ユーザーから数値を取得する入力処理は次のように書けます。

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "数を当ててみてください：",
    "これは有効な数値ではありません",
    tonumber,
    is_not_nil
)
```

これは次と同じ結果になります。

```lua
repeat
    print("数を当ててみてください：")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("これは有効な数値ではありません！")
    end
until is_valid
```

## 関数のインライン化

`is_not_nil` を一度だけ使うために定義したことに気づいてください。名前付きの関数を宣言する代わりに、他の場所で使う必要がないなら、Lua では次のように名前を省くことができます。

```lua
local double = function(number)
    return number * 2
end
```

今回の場合、これを使って次のようにできます。

```lua
validated_read(
    "数を当ててみてください：",
    "これは有効な数値ではありません",
    tonumber,
    function(nb) return nb ~= nil end
)
```

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

    user_answer = validated_read(
        "続けますか？yes / no",
        "これは有効な回答ではありません。",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("さようなら！")
```

</div>
