## 字符串键

当键是一个看起来像变量名的 `string` 时，你可以去掉方括号和引号：

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## 用 `.` 访问键

读取或修改值时也有同样的快捷方式：`player.name` 等同于 `player["name"]`。

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

你从第一章开始就一直在使用它：在 `io.read()` 中，`io` 是 Lua 内置的 `table`，而 `read` 是它的键之一！

<div class="note">

`player.key` 和 `player[key]` 并不相同！前者查找的是键 `"key"`，后者使用的是变量 `key` 的值：

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
`table.concat` 函数会把列表中的所有值连接成一个 `string`。你可以给它一个分隔符，放在值之间。这在打印列表时非常有用，因为单独的 `print` 不会显示表里面有什么：

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
`table.insert` 函数会在列表末尾添加一个值。如果你给它一个位置，值就会被插入到那里，后面的值会向后移动一位：

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
`table.remove` 函数会删除某个位置上的值，并移动后面的值，这样列表中就不会有空洞。如果没有给位置，它会删除最后一个值。它还会返回被删除的值：

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
`table.sort` 函数会对列表排序，从最小值到最大值，对于 `string` 则按字母顺序。它会修改列表本身，不会创建新列表：

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

要以另一种顺序排序，你可以给它一个函数。这个函数接收两个值，并且如果第一个值应该排在第二个值之前，就必须返回 `true`：

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

你可以在这里查看所有 `table` 函数：<https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
