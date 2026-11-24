これで次のことができるようになりました。

1. ランダムな数値を生成する
2. ユーザー入力を受け取る
3. ループする

次は、ユーザーの推測が秘密の数値より小さいか大きいかを伝えて、ユーザーを助ける必要があります。

そのために、これからよく使うことになる新しいキーワード、`if` を使います。

`if` が行うのは、基本的に条件に応じてコードを実行することです。ループは繰り返すために条件を確認し、`if` は一度だけ実行するために条件を確認します。

たとえば、ユーザーが `42` を入力したかを確認し、そうなら特別なメッセージで挨拶できます。
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("いい選択です！")
end
```

</div>

ゲームでは

```lua
if number_to_guess < user_guess then
    print("もっと小さい数です！")
end

if number_to_guess > user_guess then
    print("もっと大きい数です！")
end
```

数値が小さいか大きいかを確認し、それに応じてユーザーを助けるメッセージを表示します。

ユーザーが正しい数値を見つけた場合、次の理由でループが停止します。

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("もっと小さい数です！")
end

if number_to_guess > user_guess then
    print("もっと大きい数です！")
end

if number_to_guess == user_guess then
    print("正解！")
end
```

</div>

これができたら、あとは良い結末を作るだけです……
