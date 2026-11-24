```lua
number_to_guess = random(1, 100)
```

Bone, do ni ĵus vidis, ke vi povas meti datumojn en variablon... sed kio estas tio???

Por kompreni, kion ĉi tio faras, ni devas kompreni `function`-ojn. Ĝi estis unu el la datumtipoj, kiujn mi ne klarigis dum la unua linio, do jen ĝi:

Vi povas vidi funkciojn kiel specon de maŝino, kiu prenas iun enigon, faras ion kun ĝi, kaj poste donas al vi eligon.

## Sed kion ĝi faras kun ĝi??

Nu, en Lua ekzistas iuj operaciiloj, kiuj ebligas al vi fari specialajn aferojn:

- `+`: Adicii 2 nombrojn. (ekz.: `1 + 2` => `3`)
- `-`: Subtrahi unu nombron de alia. (ekz.: `3 - 2` => `1`)
- `*`: Multipliki 2 nombrojn. (ekz.: `3 * 2` => `6`)
- `/`: Dividi unu nombron per alia. (ekz.: `6 / 2` => `3`)

Tial, ni povas fari la jenajn aferojn kun niaj variabloj
```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

`the_best_number` estos 42 kaj `my_grandpa_age` estos 84.

## Reen al niaj funkcioj!

Ekzemple, funkcio povus preni nombron kaj redoni al vi la nombron `+ 1`.

Funkcio, kiel variablo, povas esti uzata, kiam vi konas ĝian nomon. SED! Kiam vi volas, ke ĝi faru sian maŝinaron kaj donu al vi la eligon, vi devas meti parentezojn post la nomo (ekz. `my_function()`). Ĉi tio plenumos funkcion nomatan `my_function`. Sed ĉi tiu funkcio ne havas enigon! Por aldoni enigon al ĝi, vi povas meti la valorojn, kiujn vi volas, inter la parentezoj kaj apartigi ilin per `,`.

### Ekzemplo

```lua
day = 4
month = "Aprilo"
year = 1984
calculate_tomorrows_date(day, month, year)
```
kio samas al
```lua
calculate_tomorrows_date(4, "Aprilo", 1984)
```

## Reen al nia dua linio

```lua
number_to_guess = random(1, 100)
```

Do ĉi tiu linio plenumas la funkcion `random` kun enigo: `1` kaj `100`, kaj metas la eligon de la komputado en `number_to_guess`. Ni neniam difinis, kion `random` faras. Sed Lua venas kun iuj enkonstruitaj funkcioj por helpi programistojn! Kaj en nia kazo, ekzistas funkcio `random`, kiu volas 2 nombrojn kiel enigon kaj kiu generas hazardan nombron inter la unua nombro (en nia kazo `1`) kaj la dua nombro (en nia kazo `100`).

Perfekte! Ni nun generis la nombron, kiun la uzanto devas diveni!
