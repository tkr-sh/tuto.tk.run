```lua
number_of_tries = 0
```

Ĉi tio estas la unua linio, kiun ni vidas en la programo.

Kutime, kiam vi volas kompreni, kion programo faras, vi legos ĝin linio post linio, kiel vi legus normalan anglan tekston, do ni komencas ĉi tie.


## Bone, sed kion faras ĉi tiu linio?

En programlingvoj vi ofte havas la koncepton de "variablo". Variablo estas kiel etikedita skatolo, kiu entenas datumojn. Post kiam vi kreas variablon, vi povas legi ĝian entenon - ekvivalente al malfermi skatolon - kaj vi eĉ povas ŝanĝi ĝin (de la nomo "variablo") - ekvivalente al meti novan valoron en la skatolon.

### Sed kiujn datumojn, vi eble demandos?

En Lua ekzistas 6[^1] tipoj de datumoj, kiujn vi nun bezonas scii:
- `number`: Nombro kiel en matematiko! Vi povas deklari nombron simple skribante ĝin: `123`.<br/> **Noto**: por nombroj kun decimaloj (ekz. `0.5`), uzu `.` (punkton) kaj ne `,` (komon)
- `boolean`: Aŭ `true` aŭ `false`. (ekz. "Ĉu 2 egalas al 3?" => `false`).
- `nil`: Malplena. La celo de `nil` estas diri, ke vi ne havas valoron.
- `string`: Grupo de literoj, nombroj aŭ simboloj (ekz. `Hello!`). Por deklari ĉenon, vi devas meti citilojn (`'` aŭ `"`) ĉirkaŭ la enhavo (ekz. por havi la ĉenon `Hello!` vi devas skribi `"Hello!"`)
- `table`: Ni vidos ĝin poste!
- `function`: Ni vidos ĝin poste!

Kiam vi volas asigni datumojn al variablo, vi nur devas skribi:
```lua
the_name_of_the_variable = the_value
```
`the_name_of_the_variable` povas esti io ajn. Krom `true`, `false`, `nil` (kaj aliaj, kiujn ni vidos poste)

`the_value` estas valida valoro kiel priskribite supre.

Jen kelkaj ekzemploj:
```lua
a_negative_number = -1
a_big_number = 100000000
grandpa_name = "Johano"
no_value = nil
is_it_raining = false
```

Vi ankaŭ povas asigni la entenon de variablo al alia variablo:

```lua
grandpa_name = "Johano"
my_grandpa_friend_name = grandpa_name
```

En ĉi tiu kunteksto, kaj `grandpa_name` kaj `my_grandpa_friend_name` estos la ĉeno `Johano`.

## Reen al nia unua linio

```lua
number_of_tries = 0
```

Do ĉi tiu linio simple metas la nombron `0` en la variablon `number_of_tries`. Kaj tio vere havas sencon! Ĉar komence de la programo, la uzanto ankoraŭ ne provis diveni la nombron! Kaj kiel la nomo `number_of_tries` indikas, ĉi tiu variablo entenos en la futuro la nombron da provoj, kiujn la uzanto faris!



[^1]: 

<div class="for-technical">

Nu, fakte ekzistas 8 tipoj, sed por nun nur 6 gravas :). <https://www.lua.org/pil/2.html>

</div>
