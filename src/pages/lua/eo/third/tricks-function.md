## Enliniaj funkcioj

Ĝis nun, ni skribis funkciojn tiel:

```lua
function double(number)
    return number * 2
end
```

Ĉi tio estas sukero. Kion Lua vere faras estas krei funkcion sen nomo kaj konservi ĝin en variablo nomata `double`:

```lua
double = function(number)
    return number * 2
end
```

Ambaŭ versioj estas precize la samaj. Ĝuste tial ni povis doni `tonumber` al `validated_read`: funkcio estas simple valoro konservita en variablo, kiel nombro aŭ ĉeno.


## Voki funkciojn per `:`

Por igi ĉenon minuskla, ni skribis `string.lower(user_answer)`.
Sed, kiam vi uzas funkcion el la biblioteko `string` aŭ `table`, kiu prenas kiel unuan argumenton `string` aŭ `table`, vi povas skribi tion per la jena sintakso: `user_answer:lower()`:

<div class="runner">

```lua
local answer = "JES"

print(string.lower(answer))
print(answer:lower())
```

</div>

La `:` prenas la valoron maldekstre kaj donas ĝin al la funkcio kiel ĝian unuan argumenton. Por ĉeno, Lua serĉas la funkcion en la tabelo `string`, do `answer:lower()` fariĝas `string.lower(answer)`.

<div class="note">

Ĉi tio ne funkcias sur ĉeno skribita rekte en la kodo: `"JES":lower()` estas sintaksa eraro, ĉar Lua akceptas `:` nur post variabla nomo aŭ post parentezoj. Por igi ĝin funkcii, metu la ĉenon inter parentezoj:

```lua
print(("JES"):lower())
-- Ĉi tio eraras:
print("JES":lower())
```

</div>

## Sen parentezoj

Kiam funkcio estas vokata kun unu sola argumento, kaj ĉi tiu argumento estas `string` aŭ `table` skribita rekte en la kodo, la parentezoj estas laŭvolaj:

<div class="runner">

```lua
print "Saluton!"
print("Saluton!")
```

</div>

Vi vidos ĉi tion en la kodo de aliaj homoj. En ĉi tiu kurso ni konservas la parentezojn, ĉar ili klare montras, ke funkcio estas vokata.

## Redoni plurajn valorojn

Funkcio povas redoni pli ol unu valoron. Vi apartigas ilin per `,` post la `return`, kaj vi ricevas ilin en pluraj variabloj, ankaŭ apartigitaj per `,`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

Se vi donas malpli da variabloj ol da valoroj, la kromaj valoroj simple estas forĵetataj. Se vi donas pli, la kromaj variabloj estas `nil`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

Por konservi nur la unuan valoron de `function`, metu la vokon inter parentezoj:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## Mankantaj kaj kromaj argumentoj

Lua ne kontrolas la nombron da argumentoj, kiujn vi donas al `function`. Argumento, kiu mankas, estas `nil`, kaj argumento, kiu ne estas atendita, estas ignorata:

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## Laŭvolaj argumentoj

Ĉar mankanta argumento estas `nil`, ni povas kontroli ĝin kaj uzi defaŭltan valoron anstataŭe. Tio igas la argumenton laŭvola:

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Saluton"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Bonan vesperon")
```
