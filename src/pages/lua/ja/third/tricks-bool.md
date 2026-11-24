## 何が `true` とみなされる？

条件では、Lua は `true` と `false` だけを受け付けるわけではありません。どんな値でも使えます。ルールは簡単です。`false` と `nil` は偽とみなされ、**それ以外のすべて**は真とみなされます。

別のプログラミング言語を知っている人は注意してください。Lua では、`0` と空文字列 `""` も真とみなされます！

<div class="runner">

```lua
if 0 then
    print("0 は真とみなされます")
end

if "" then
    print("空文字列は真とみなされます")
end

if nil then
    print("これは決して表示されません")
end
```

</div>

## `== nil` なしで `nil` を確認する

このルールのおかげで、変数をそのまま条件として使うことで、値があるかどうかを確認できます。

<div class="runner">

```lua
local name = nil

if name then
    print("こんにちは " .. name)
else
    print("名前がありません")
end
```

</div>

`if name then` は `if name ~= nil then` の短い書き方で、`if not name then` は `if name == nil then` の短い書き方です。

<div class="note">

変数が `false` を含む可能性がある場合、これらは完全には同じではありません。`false` は `nil` ではありませんが、条件では偽とみなされます。

</div>

## `not`

`not` キーワードは常に本物のブール値を返します。`false` と `nil` には `true` を、それ以外のすべてには `false` を返します。

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or` と `and` は値の一方を返す

`or` と `and` はブール値で見ましたが、どんな値でも機能し、常にブール値を返すわけではありません。2 つの値のうち一方を返します。

- `a or b` は、`a` が真とみなされるなら `a`、そうでなければ `b` を返します。
- `a and b` は、`a` が偽とみなされるなら `a`、そうでなければ `b` を返します。

<div class="runner">

```lua
print(nil or "デフォルト")
print("値" or "デフォルト")

print(nil and "次")
print("値" and "次")
```

</div>

`true` と `false` の場合、これは以前に見た表とまったく同じになります。

## `or` でデフォルト値を設定する

これが、`or` が変数にデフォルト値を与えられる理由です。オプション引数で行ったのと同じです。

<div class="runner">

```lua
local name = nil
name = name or "ゲスト"

print(name)
```

</div>

`name` に値があれば、それが保持されます。`name` が `nil` なら、`"ゲスト"` になります。

## 2 つの値から選ぶ

`and` と `or` を一緒に使うことで、条件に応じて 2 つの値から 1 行で選べます。

<div class="runner">

```lua
local score = 70
local result = score > 50 and "あなたの勝ちです！" or "あなたの負けです！"

print(result)
```

</div>

これは次と同じです。

```lua
local result
if score > 50 then
    result = "あなたの勝ちです！"
else
    result = "あなたの負けです！"
end
```

<div class="note">

最初の値が `false` または `nil` の場合、このトリックは機能しません。`condition and false or "その他"` は常に `"その他"` を返します。その場合は通常の `if` を使ってください。

</div>

## `and` はできるだけ早く止まる

`and` の最初の値が偽とみなされる場合、Lua は 2 つ目の値を見ることさえしません。コースの最初に `nil` と数値を比較したときのエラーを覚えていますか？ `and` はそれを避けるのに役立ちます。

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("推測値は 10 より大きいです")
end

print("エラーなし！")
```

</div>

`user_guess ~= nil` が `false` なので、`user_guess > 10` は決して実行されません。同様に、`or` は最初の値が真とみなされる場合、2 つ目の値を見ません。

## テーブルの中の `nil`

テーブルに存在しないキーは、エラーなく `nil` を返します。そして、テーブルからキーを削除するには、それに `nil` を代入します。

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

これは `score` のようなキーを削除する正しい方法ですが、リスト内の値には使えません。見てきたように、穴ができてしまいます。リストには `table.remove` を使ってください。

</div>
