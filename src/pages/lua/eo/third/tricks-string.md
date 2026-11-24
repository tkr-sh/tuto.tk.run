## Citiloj

Ĉeno povas esti skribita per duoblaj citiloj `"..."` aŭ unuoblaj citiloj `'...'`. Ambaŭ donas precize la saman ĉenon. Ĉi tio utilas, kiam via teksto enhavas unu el la du:

<div class="runner">

```lua
print("Estas bela tago")
print('Li diris "saluton"')
```

</div>

## Plurliniaj ĉenoj

Vi ne povas simple aldoni linisalton en la mezo de `"..."`-ĉeno tiel:

<div class="runner">

```lua
print("linio 1
linio 2")
```
</div>

Lua simple ne permesas linisalton tiel en ĉeno, kiam vi skribas ĝin per `"` aŭ `'`.

TAMEN! Vi povas fari tion anstataŭigante `"` aŭ `'` per `[[`/`]]`

<div class="runner">

```lua
print([[linio1
linio2]])
```

</div>

## Longo

La operaciilo `#`, kiun ni uzis sur tabeloj, ankaŭ funkcias sur ĉenoj:

<div class="runner">

```lua
print(#"saluton")
print(#"saluton mondo")
```

</div>

## `string.sub`
La funkcio `string.sub` permesas krei novan `string`, el parto de la `string`, specifante la komencan kaj finan indeksojn:

<div class="runner">

```lua
print(("saluton"):sub(2, 4))
-- Samkiel
-- print(string.sub("saluton", 2, 4))
```

</div>

## `string.rep`
La funkcio `string.rep` permesas ripeti `string` plurajn fojojn:

<div class="runner">

```lua
print(("saluton"):rep(3))
```

</div>

## `string.reverse`
La funkcio `string.reverse`, kiel ĝia nomo sugestas, inversigas `string`:

<div class="runner">

```lua
print(("saluton"):reverse())
```

</div>

## `string.gsub`
La funkcio `string.gsub` anstataŭigas `string` per alia, kaj ankaŭ presas kiom da fojoj ĝi estis anstataŭigita:

<div class="runner">

```lua
print(("saluton"):gsub("ut", "it"))
print(("saluton"):gsub("a", "e"))
```

</div>

---

Vi povas kontroli ĉiujn funkciojn de `string` ĉi tie: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
