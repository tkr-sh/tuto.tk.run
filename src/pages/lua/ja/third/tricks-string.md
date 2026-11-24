## 引用符

文字列は二重引用符 `"..."` または単一引用符 `'...'` で書けます。どちらもまったく同じ文字列になります。これは、テキストにどちらかの引用符が含まれているときに便利です。

<div class="runner">

```lua
print("今日はいい天気です")
print('彼は「こんにちは」と言った')
```

</div>

## 複数行の文字列

次のように、`"..."` 文字列の途中に単純に改行を入れることはできません。

<div class="runner">

```lua
print("1 行目
2 行目")
```
</div>

`"` または `'` で文字列を書くとき、Lua はそのような改行を許しません。

でも！ `"` または `'` を `[[`/`]]` に置き換えれば可能です。

<div class="runner">

```lua
print([[1 行目
2 行目]])
```

</div>

## 長さ

テーブルで使った `#` 演算子は文字列にも使えます。

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
`string.sub` 関数は、開始と終了のインデックスを指定して、`string` の一部から新しい `string` を作れます。

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- 次と同じ
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
`string.rep` 関数は、`string` を複数回繰り返せます。

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
`string.reverse` 関数は、その名前のとおり `string` を逆にします。

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
`string.gsub` 関数は、ある `string` を別の `string` に置き換え、何回置き換えたかも表示します。

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

すべての `string` 関数はこちらで確認できます：<https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
