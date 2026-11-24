## その他の演算子

`+`、`-`、`*`、`/` はすでに知っています。Lua には `number` 用の演算子がさらに 3 つあります。

|演算子|行うこと|
|-|-|
|`//`|割り算をして小数部分を切り捨てる|
|`%`|割り算の余りを返す|
|`^`|累乗する|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

`%` 演算子は、`number` が偶数か奇数かを知るのに非常に便利です。`number` を `2` で割った余りが `0` なら偶数です。

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## 整数の `number` と小数の `number`

Lua には 2 種類の `number` があります。`5` のような整数の `number` と、`5.0` や `3.14` のような小数の `number` です。ほとんどの場合、これを意識する必要はありませんが、なぜ一部の結果が `.0` 付きで表示されるのかを説明できます。

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

`/` 演算子は、結果が割り切れても常に小数の `number` を返します。`5` と `5.0` は依然として等しいです。

<div class="note">

小数の `number` は常に正確とは限りません！ コンピューターは `0.1` のような `number` を完全には保存できないため、一部の比較は驚くべき結果になります。

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

小数の `number` を使った計算結果に `==` を使うのは避けてください。

</div>

## `number` の他の書き方

非常に大きい、または非常に小さい `number` は、`e` を使って書けます。これは「10 の何乗」を意味します。[16 進数](https://en.wikipedia.org/wiki/Hexadecimal)で `number` を書くこともできます。その場合は `0x` で始めます。

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## `number` と文字列の変換

`tostring` 関数は `number` を `string` に変換し、すでに使った `tonumber` はその逆を行います。

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua は可能な場合、自分で変換も行います。`number` のように見える `string` は計算の中で `number` になり、`number` は `..` で `string` になります。

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` と `math.ceil`
`math.floor` 関数は `number` を切り捨て、`math.ceil` は切り上げます。

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

最も近い整数の `number` に四捨五入するには、`math.floor` を使う前に `0.5` を足します。

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` と `math.min`
`math.max` 関数は渡した `number` の中で最大のものを、`math.min` は最小のものを返します。

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
`math.abs` 関数は `number` の符号を取り除きます。これは 2 つの `number` の距離を求めるのに便利です。

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
`math.sqrt` 関数は `number` の平方根を返します。

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

すべての `math` 関数はこちらで確認できます：<https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
