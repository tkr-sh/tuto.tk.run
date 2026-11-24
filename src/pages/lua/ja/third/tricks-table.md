## 文字列キー

キーが変数名のように見える `string` の場合、角括弧と引用符を省略できます。

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## `.` でキーにアクセスする

値を読み取ったり変更したりするときにも同じショートカットがあります。`player.name` は `player["name"]` と同じです。

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

これは第 1 章からずっと使ってきました。`io.read()` では、`io` は Lua 組み込みの `table` で、`read` はそのキーの 1 つです！

<div class="note">

`player.key` と `player[key]` は同じではありません！ 前者はキー `"key"` を探し、後者は変数 `key` の値を使います。

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }
local key = "score"

print(player.key) -- nil
print(player[key]) -- 42
```
</div>

</div>

## `table.concat`
`table.concat` 関数は、リストのすべての値を 1 つの `string` に結合します。値の間に置く区切り文字を渡すこともできます。これはリストを表示するのに非常に便利です。`print` だけではテーブルの中身を表示できないからです。

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
`table.insert` 関数は、リストの末尾に値を追加します。位置を渡すと、その位置に値が挿入され、それ以降の値は 1 つ後ろに移動します。

<div class="runner">

```lua
local names = { "bob", "carol" }

table.insert(names, "dave")
print(table.concat(names, ", "))

table.insert(names, 2, "alice")
print(table.concat(names, ", "))
```

</div>

## `table.remove`
`table.remove` 関数は、ある位置の値を削除し、それ以降の値を移動させるので、リストに穴が残りません。位置を渡さない場合は、最後の値を削除します。削除した値も返します。

<div class="runner">

```lua
local names = { "bob", "alice", "carol" }

print(table.remove(names, 1))
print(table.concat(names, ", "))

print(table.remove(names))
print(table.concat(names, ", "))
```

</div>

## `table.sort`
`table.sort` 関数は、リストを最小値から最大値へ、`string` の場合はアルファベット順に並べ替えます。リスト自体を変更し、新しいリストは作りません。

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

別の順序で並べ替えるには、関数を渡せます。この関数は 2 つの値を受け取り、最初の値を 2 つ目の値より前に置くべきなら `true` を返さなければなりません。

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

すべての `table` 関数はこちらで確認できます：<https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
