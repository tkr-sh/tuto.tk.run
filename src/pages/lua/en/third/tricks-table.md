## String keys

When a key is a `string` that looks like a variable name, you can remove the brackets and the quotes:

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## Accessing a key with `.`

The same shortcut exists to read or change a value: `player.name` is the same as `player["name"]`.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

You have been using this since the first chapter: in `io.read()`, `io` is a builtin-in lua `table` and `read` is one of its keys!

<div class="note">

`player.key` and `player[key]` are not the same! The first one looks for the key `"key"`, the second one uses the value of the variable `key`:

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
The `table.concat` function joins all the values of a list into a single `string`. You can give it a separator to put between the values. This is very useful to print a list, because `print` alone doesn't show what is inside a table:

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
The `table.insert` function adds a value at the end of a list. If you give it a position, the value is inserted there and the following values are moved by one:

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
The `table.remove` function removes the value at a position and moves the following values, so that there is no hole in the list. Without a position, it removes the last value. It also gives back the value that was removed:

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
The `table.sort` function sorts a list, from the smallest to the biggest value, or in alphabetical order for `string`s. It changes the list itself and doesn't create a new one:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

To sort in another order, you can give it a function. This function receives two values and must return `true` if the first one should be placed before the second one:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

You can check all the `table` functions here: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
