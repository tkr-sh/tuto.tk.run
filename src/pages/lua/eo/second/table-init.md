Kio se nun ni povus havi historion de niaj poentoj, ĝis ni finos ludi? Historion de ĉio, kio okazis? Kiel ni farus tion?

Via unua aliro eble estus difini plurajn variablojn tiel:

```lua 
score_first_game = nil
score_second_game = nil
score_third_game = nil
...
```

Sed ĉi tio ne estas ĝusta aliro. Ĉi tio ne estas daŭrigebla, se la uzanto ludas multajn fojojn. Ĉi tio malfacile ligiĝas pure kun buklo.

Por solvi tion, ni bezonos novan datumtipon: `table`.

## Kio estas tabelo?

`table` estas Lua-tipo, kiu permesas al ni konservi arbitre longan aŭ mallongan kvanton da datumoj. Ĉiu Lua-tabelo asignas ŝlosilon al valoro en la formato `[key] = value`, tiel:

```lua
my_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

Ni povas aliri la valoron de tabelo referencante la ŝlosilon kaj farante `my_table[my_key]`. Ekzemple:

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(my_table["b"])
```

</div>

Kaj ni povas simple aldoni novan valoron al nia tabelo asignante neekzistantan ŝlosilon al valoro tiel:

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

my_table["d"] = 5
print(my_table["d"])
```

</div>

Vi ankaŭ povas skribi neniun ŝlosilon, kaj en ĉi tiu kazo, la ŝlosiloj komenciĝos de unu kaj poste pliiĝos:


<div class="runner">

```lua
my_table = { "a", "b", "c", "z" }
-- Samkiel
-- my_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(my_table[2])
print(my_table[4])
```

</div>
