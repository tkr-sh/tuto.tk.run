## 문자열 키

키가 변수 이름처럼 보이는 `string`일 때는 대괄호와 따옴표를 제거할 수 있습니다.

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## `.`으로 키에 접근하기

값을 읽거나 변경할 때도 같은 단축 방법이 있습니다. `player.name`은 `player["name"]`과 같습니다.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

이것은 1장부터 계속 사용해 왔습니다. `io.read()`에서 `io`는 Lua 내장 `table`이고 `read`는 그 키 중 하나입니다!

<div class="note">

`player.key`와 `player[key]`는 같지 않습니다! 전자는 `"key"`라는 키를 찾고, 후자는 변수 `key`의 값을 사용합니다.

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
`table.concat` 함수는 리스트의 모든 값을 하나의 `string`으로 결합합니다. 값 사이에 넣을 구분자를 전달할 수도 있습니다. 이것은 리스트를 출력할 때 매우 유용합니다. `print`만으로는 테이블 안에 무엇이 있는지 보여 주지 못하기 때문입니다.

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
`table.insert` 함수는 리스트 끝에 값을 추가합니다. 위치를 전달하면 그 위치에 값이 삽입되고 뒤의 값들은 하나씩 뒤로 밀립니다.

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
`table.remove` 함수는 특정 위치의 값을 제거하고 뒤의 값들을 이동시켜 리스트에 구멍이 생기지 않게 합니다. 위치를 전달하지 않으면 마지막 값을 제거합니다. 제거한 값도 돌려줍니다.

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
`table.sort` 함수는 리스트를 가장 작은 값부터 가장 큰 값 순으로, `string`의 경우 알파벳 순으로 정렬합니다. 리스트 자체를 변경하며 새 리스트를 만들지 않습니다.

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

다른 순서로 정렬하려면 함수를 전달할 수 있습니다. 이 함수는 두 값을 받고, 첫 번째 값이 두 번째 값보다 앞에 와야 하면 `true`를 돌려주어야 합니다.

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

모든 `table` 함수는 여기에서 확인할 수 있습니다: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
