## Строковые ключи

Когда ключ — это `string`, которая выглядит как имя переменной, вы можете убрать квадратные скобки и кавычки:

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## Доступ к ключу с помощью `.`

Такое же сокращение существует для чтения или изменения значения: `player.name` — то же самое, что `player["name"]`.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

Вы пользовались этим с самой первой главы: в `io.read()` `io` — это встроенная `table` Lua, а `read` — один из её ключей!

<div class="note">

`player.key` и `player[key]` — не одно и то же! Первое ищет ключ `"key"`, второе использует значение переменной `key`:

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
Функция `table.concat` объединяет все значения списка в одну `string`. Вы можете передать ей разделитель, который ставится между значениями. Это очень полезно для вывода списка, потому что один `print` не показывает, что находится внутри таблицы:

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
Функция `table.insert` добавляет значение в конец списка. Если вы передадите ей позицию, значение вставляется туда, а последующие значения сдвигаются на одну позицию:

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
Функция `table.remove` удаляет значение в позиции и сдвигает последующие значения так, чтобы в списке не было дыры. Без позиции она удаляет последнее значение. Она также возвращает удалённое значение:

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
Функция `table.sort` сортирует список от наименьшего значения к наибольшему или в алфавитном порядке для `string`. Она изменяет сам список и не создаёт новый:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

Чтобы отсортировать в другом порядке, вы можете передать ей функцию. Эта функция получает два значения и должна вернуть `true`, если первое должно быть расположено перед вторым:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

Все функции `table` вы можете посмотреть здесь: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
