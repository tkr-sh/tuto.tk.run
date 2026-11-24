Alia ŝlosilo por labori kun tabeloj estas trairi ilin. Ni diru, ke ni havas en nia ludo la tabelon `scores` (`score` estante la `number_of_tries`, kiun uzanto bezonis por trovi la nombron). Kiel vi montras ĉiun poenton por ĉiu ludo?

## `for index`
Unue, vi eble memoras la ŝlosilvorton `for`, pri kiu ni rapide parolis antaŭe:

<blockquote>

`for`: La lasta buklo estas la buklo `for`, kaj ĝi estas iom malsama, ĉar vi jam scias, kiom da fojoj vi volas ripeti. Kaj vi difinas, de kiu nombro ĝis kiu nombro vi volas ripeti. Fine de ripeto, ĝi aŭtomate pliigos `age` je `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

Se ni ligas ĉi tion kun nia nova scio pri la operaciilo `#`, ni do povas sukcese trairi tabelon!

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for key = 1, #scores do
    print(key .. ": " .. scores[key])
end
```
</div>

Ĉi tio funkcias bonege! Sed povas esti iom ripete ĉiam skribi `scores[key]` por aliri la valoron, precipe se vi ne zorgas pri la ŝlosilo, aŭ se vi ne planas uzi la tabelon plu. Bonŝance Lua havas alian manieron trakti tion.

## `ipairs`

`ipairs` estas funkcio, kiun ni uzos kun la ŝlosilvorto `for` por povi rapide aliri ŝlosilon kaj valoron sen devi referenci la nomon de la tabelo! Ĝia sintakso iom malsamas al la antaŭa `for`-buklo kvankam ĝi ankoraŭ uzas la ŝlosilvorton `for`:

```lua
for my_key, my_value in ipairs(my_table) do 
    -- Faru ion!
end
```

En ĉi tiu ekzemplo, ni supozas, ke ni havas `table` nomatan `my_table`. Ĉiu koda linio inter la `for` kaj la `end` plenumiĝos kun `my_key` kaj `my_value` estante la ŝlosilo kaj la asociata valoro de `my_table`. 

Kiel la antaŭa buklo `for`, ĝi estas ordigita laŭ kreskanta ordo. Tio signifas, ke ĝi komenciĝas ĉe `1`, kaj pliiĝas je `1` ĉe ĉiu ripeto. Sed ĝi ne funkcias, kiam la ŝlosiloj estas `string`-oj aŭ se estas mankoj inter nombroj. Se tio estas la kazo, ni devus uzi `pairs`.

## `pairs`

`pairs` estas kiel `ipairs`, kun la diferenco, ke ĝi funkcias kun ŝlosiloj, kiuj ne estas sinsekvaj entjeroj komencante de `1`. La malavantaĝo estas, ke la ordo, en kiu la ŝlosiloj venas, estas nefidinda:

<div class="runner">

```lua
my_table = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for key, value in pairs(my_table) do
    print(key)
    print(value)
end
```

</div>

La ordo, en kiu ŝlosiloj/valoroj estas presitaj, estas nefidinda kaj povas varii de unu rulo al la sekva. Tamen, ĝi havas la avantaĝon permesi al ni trairi ĉiun tabelon!

## Reen al nia ludo
En nia kazo, ni simple volas konservi la `number_of_tries` por ĉiu ludo. Tial, ne necesas propraj ŝlosiloj, ni povas simple havi indeksojn komenciĝantajn de `1`, kaj pliiĝantajn je `1` ĉiufoje, respondantajn al la ludnumero!
