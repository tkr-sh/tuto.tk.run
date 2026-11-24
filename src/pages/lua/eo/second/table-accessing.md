Utila informo, kiun ni bezonas dum laborado kun tabeloj, estas "kiom da elementoj ĝi havas?" Ekzemple se mi volas akiri la lastan elementon!

Tio eblas per speciala operaciilo: `#`. Skribante `#` kaj poste la nomon de la tabelo aŭ la tabelon rekte, vi povas akiri la totalan grandon de la tabelo:


<div class="runner">

```lua
my_table = { "a", "b", "c" }

print(#my_table)
my_table[4] = "d"
print(#my_table)
```

</div>

Ĉi tio estas aparte utila, se mi volas scii la elementon de tabelo kun la "lasta" indekso, aŭ se mi volas aldoni novan elementon ĉe la fino: `my_table[#my_table]` estas la valoro, kies ŝlosilo estas la totala longeco de la tabelo, tio signifas la lasta en plej multaj kazoj!

<div class="runner">

```lua
my_table = { "a", "b", "c" }

print(my_table[#my_table])
my_table[#my_table + 1] = "d"
print(my_table[#my_table])
```

</div>


### Rando-kazoj

La operaciilo `#` estas fidinda nur kiam la ŝlosiloj de tabelo estas la tutaj nombroj `1, 2, 3, ...` sen mankoj komencante de `1`. Ŝlosiloj, kiuj estas ĉenoj aŭ buleaj, ne estas kalkulataj, kaj se nombro mankas en la mezo, la rezulto estas neantaŭvidebla:

<div class="runner">

```lua
-- Fidinda: la ŝlosiloj estas 1, 2, 3
print(#{ "a", "b", "c" }) -- 3
print(#{ [1] = "a", [2] = "b", [3] = "c" }) -- 3

-- Ne kalkulataj: la ŝlosiloj estas ĉenoj
print(#{ ["a"] = 1, ["b"] = 2, ["c"] = 3 }) -- 0

-- Ne fidinda: mankas indekso 2
print(#{ [1] = "a", [3] = "b", [4] = "c" }) -- 1 aŭ 4, ne fidu ĝin
```

</div>
