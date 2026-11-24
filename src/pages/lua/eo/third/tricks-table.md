## Ĉenaj ŝlosiloj

Kiam ŝlosilo estas `string`, kiu aspektas kiel variabla nomo, vi povas forigi la krampojn kaj la citilojn:

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## Aliri ŝlosilon per `.`

La sama mallongigo ekzistas por legi aŭ ŝanĝi valoron: `player.name` samas al `player["name"]`.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

Vi uzas ĉi tion ekde la unua ĉapitro: en `io.read()`, `io` estas enkonstruita Lua-`table` kaj `read` estas unu el ĝiaj ŝlosiloj!

<div class="note">

`player.key` kaj `player[key]` ne estas la samaj! La unua serĉas la ŝlosilon `"key"`, la dua uzas la valoron de la variablo `key`:

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
La funkcio `table.concat` kunigas ĉiujn valorojn de listo en unu solan `string`. Vi povas doni al ĝi apartigilon por meti inter la valoroj. Ĉi tio tre utilas por presi liston, ĉar `print` sola ne montras, kio estas ene de tabelo:

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
La funkcio `table.insert` aldonas valoron ĉe la fino de listo. Se vi donas al ĝi pozicion, la valoro estas enmetata tie kaj la sekvaj valoroj estas movataj je unu:

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
La funkcio `table.remove` forigas la valoron ĉe pozicio kaj movas la sekvajn valorojn, por ke ne estu truo en la listo. Sen pozicio, ĝi forigas la lastan valoron. Ĝi ankaŭ redonas la valoron, kiu estis forigita:

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
La funkcio `table.sort` ordigas liston, de la plej malgranda ĝis la plej granda valoro, aŭ laŭ alfabeta ordo por `string`-oj. Ĝi ŝanĝas la liston mem kaj ne kreas novan:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

Por ordigi laŭ alia ordo, vi povas doni al ĝi funkcion. Ĉi tiu funkcio ricevas du valorojn kaj devas redoni `true`, se la unua devus esti metita antaŭ la dua:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

Vi povas kontroli ĉiujn funkciojn de `table` ĉi tie: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
