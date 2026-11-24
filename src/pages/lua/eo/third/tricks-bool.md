## Kio validas kiel `true`?

En kondiĉo, Lua ne nur akceptas `true` kaj `false`: iu ajn valoro povas esti uzata. La regulo estas simpla: `false` kaj `nil` validas kiel malvero, kaj **ĉio alia** validas kiel vero.

Atentu, se vi konas alian programlingvon: en Lua, `0` kaj la malplena ĉeno `""` validas kiel vero!

<div class="runner">

```lua
if 0 then
    print("0 validas kiel vero")
end

if "" then
    print("Malplena ĉeno validas kiel vero")
end

if nil then
    print("Ĉi tio neniam estas presita")
end
```

</div>

## Kontroli `nil` sen `== nil`

Dank' al ĉi tiu regulo, ni povas kontroli, ke variablo havas valoron, uzante ĝin rekte kiel kondiĉon:

<div class="runner">

```lua
local name = nil

if name then
    print("Saluton " .. name)
else
    print("Ne estas nomo")
end
```

</div>

`if name then` estas pli mallonga maniero skribi `if name ~= nil then`, kaj `if not name then` estas pli mallonga maniero skribi `if name == nil then`.

<div class="note">

Ĉi tiuj ne estas precize la samaj, kiam la variablo povas enhavi `false`: `false` ne estas `nil`, sed ĝi ankaŭ validas kiel malvero en kondiĉo.

</div>

## `not`

La ŝlosilvorto `not` ĉiam donas veran bulean valoron. Ĝi donas `true` por `false` kaj `nil`, kaj `false` por ĉio alia:

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or` kaj `and` redonas unu el siaj valoroj

Ni vidis `or` kaj `and` kun buleaj valoroj, sed ili funkcias kun iu ajn valoro, kaj ili ne ĉiam donas bulean valoron: ili redonas unu el siaj du valoroj.

- `a or b` donas `a` se ĝi validas kiel vero, kaj `b` alie.
- `a and b` donas `a` se ĝi validas kiel malvero, kaj `b` alie.

<div class="runner">

```lua
print(nil or "defaŭlto")
print("valoro" or "defaŭlto")

print(nil and "sekva")
print("valoro" and "sekva")
```

</div>

Kun `true` kaj `false`, ĉi tio donas precize la tabelojn, kiujn ni vidis antaŭe.

## Defaŭltaj valoroj per `or`

Ĝuste tial `or` povas doni defaŭltan valoron al variablo, kiel ni faris por laŭvolaj argumentoj:

<div class="runner">

```lua
local name = nil
name = name or "Gasto"

print(name)
```

</div>

Se `name` havas valoron, ĝi estas konservata. Se `name` estas `nil`, ĝi fariĝas `"Gasto"`.

## Elekti inter du valoroj

Uzante `and` kaj `or` kune, ni povas elekti inter du valoroj per kondiĉo, en unu sola linio:

<div class="runner">

```lua
local score = 70
local result = score > 50 and "Vi venkas!" or "Vi malvenkas!"

print(result)
```

</div>

Ĉi tio samas al:

```lua
local result
if score > 50 then
    result = "Vi venkas!"
else
    result = "Vi malvenkas!"
end
```

<div class="note">

Ĉi tiu artifiko ne funkcias, se la unua valoro estas `false` aŭ `nil`: `condition and false or "alia"` ĉiam donas `"alia"`. En ĉi tiu kazo, uzu normalan `if`.

</div>

## `and` haltas kiel eble plej frue

Kiam la unua valoro de `and` validas kiel malvero, Lua eĉ ne rigardas la duan. Ĉu vi memoras la eraron, kiun ni havis komence de la kurso, kiam ni komparis `nil` kun nombro? `and` permesas al ni eviti ĝin:

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("La diveno estas pli granda ol 10")
end

print("Neniu eraro!")
```

</div>

Ĉar `user_guess ~= nil` estas `false`, `user_guess > 10` neniam estas plenumata. Sammaniere, `or` ne rigardas sian duan valoron, kiam la unua validas kiel vero.

## `nil` en tabeloj

Ŝlosilo, kiu ne ekzistas en tabelo, donas `nil`, sen iu eraro. Kaj por forigi ŝlosilon el tabelo, vi asignas `nil` al ĝi:

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

Ĉi tio estas la ĝusta maniero forigi ŝlosilon kiel `score`, sed ne valoron en listo: kiel ni vidis, ĝi kreus truon. Por listoj, uzu `table.remove`.

</div>
