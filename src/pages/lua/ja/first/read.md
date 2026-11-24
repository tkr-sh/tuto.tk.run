```lua
user_guess = tonumber(read())
```

さて、このコードは奇妙に見えるかもしれません（関数の中に関数？）。しかし、次のように捉えられます。

```lua
read_value = read()
user_guess = tonumber(read_value)
```

起こっていることは次のとおりです。
1. Lua は右辺の値を計算しようとします
2. Lua は `tonumber` を実行しようとしますが、そのためにはすべての入力に値が必要です
3. そのため、`read()` を見た瞬間に、関数が値を生成するようにすぐ実行しようとします
4. `read()` が実行されて値を返した後、`tonumber` を実行できるようになります！

### では、`read()` は何をしているの？

`read()` はユーザーに文字列を尋ね、それを取得します。

**例を実行する前に：** このサイトのポップアップを無効にしないでください。そうしないと、再び `read()` できなくなります！



<div class="runner">

```lua
print(read())
```

</div>

### では `tonumber` は？

`read()` はユーザーから `string` を取得するので、それをランダムな数値と比較することはできません。数字以外の文字を含む可能性がある `string` と `number` を比較することはできないからです（例：`"x4%$A" > 3` は意味をなしません）。

したがって、ユーザーの入力を数値に変換する必要があり、まさにそれが `tonumber` の役割です！

この関数は、入力が有効な数値でない場合、`nil` を返します。

<div class="runner">

```lua
print(tonumber("数値ではない") == nil)
```

</div>

また、`nil` + `number` を足すことはできないので、有効な数値を入力してください！

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

ユーザーから `string` を読み取り、それを `number` に変換して 1 を足します。有効な数値でなければ、`tonumber(read())` は `nil` 値になります。
