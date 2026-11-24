## Pli da operaciiloj

Vi jam konas `+`, `-`, `*` kaj `/`. Lua havas tri aliajn operaciilojn por `number`-oj:

|Operaciilo|Kion ĝi faras|
|-|-|
|`//`|Dividas kaj forigas la decimalojn|
|`%`|Donas la reston de divido|
|`^`|Levigas al potenco|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

La operaciilo `%` tre utilas por scii, ĉu `number` estas para aŭ nepara: `number` estas para, kiam la resto de ĝia divido per `2` estas `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Tutaj `number`-oj kaj decimalaj `number`-oj

Lua havas du specojn de `number`-oj: tutajn `number`-ojn kiel `5`, kaj decimalajn `number`-ojn kiel `5.0` aŭ `3.14`. Plejofte vi ne bezonas pensi pri tio, sed ĝi klarigas, kial iuj rezultoj estas presitaj kun `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

La operaciilo `/` ĉiam donas decimalan `number`, eĉ kiam la rezulto estas ronda. `5` kaj `5.0` tamen estas egalaj.

<div class="note">

Decimalaj `number`-oj ne ĉiam estas precizaj! Komputiloj ne povas konservi `number` kiel `0.1` perfekte, do iuj komparoj donas surprizajn rezultojn:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Evitu uzi `==` sur la rezulto de kalkulo kun decimalaj `number`-oj.

</div>

## Aliaj manieroj skribi `number`-ojn

Tre grandaj aŭ tre malgrandaj `number`-oj povas esti skribitaj per `e`, kio signifas "oble 10 potencigita". Vi ankaŭ povas skribi `number`-ojn en [deksesuma sistemo](https://en.wikipedia.org/wiki/Hexadecimal) komencante per `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Konverti `number`-ojn kaj ĉenojn

La funkcio `tostring` konvertas `number` al `string`, kaj `tonumber`, kiun ni jam uzis, faras la malon:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua ankaŭ faras la konverton mem, kiam ĝi povas: `string`, kiu aspektas kiel `number`, fariĝas `number` en kalkulo, kaj `number` fariĝas `string` per `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` kaj `math.ceil`
La funkcio `math.floor` rondigas `number` malsupren, kaj `math.ceil` rondigas ĝin supren:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

Por rondigi al la plej proksima tuta `number`, aldonu `0.5` antaŭ uzi `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` kaj `math.min`
La funkcio `math.max` donas la plej grandan el la `number`-oj, kiujn vi donas al ĝi, kaj `math.min` la plej malgrandan:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
La funkcio `math.abs` forigas la signon de `number`. Ĉi tio utilas por akiri la distancon inter du `number`-oj:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
La funkcio `math.sqrt` donas la kvadratan radikon de `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

Vi povas kontroli ĉiujn funkciojn de `math` ĉi tie: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
