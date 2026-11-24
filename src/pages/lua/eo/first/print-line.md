```lua
print("Provu diveni la hazardan nombron inter 1 kaj 100!")
```

Ĉi tiu linio ankaŭ estas voko al funkcio! Ni povas vidi tion, ĉar ni havas la nomon `print` sed kun parentezoj post ĝia nomo!

Ĉi tiu funkcio prenas ĉenon kiel sian unuan kaj solan parametron. Sed... ni neniam metas la eligon en variablon.

## Kial?

Ĉar `print` ne redonas valoron. `print` estas speciala funkcio, kiu montras al la uzanto la datumojn, kiujn ĝi ricevas en sia enigo.

Ekzemple, ni reprenu nian antaŭan ekzemplon:

```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

Kiel mi sciu, kio estas la valoro de `my_grandpa_age`? Mi povus diveni per kalkulado en mia kapo, sed kiam la programo fariĝas pli kompleksa, tio ne ĉiam eblas.

Tial, ni povas simple fari 

```lua
print(my_grandpa_age)
```

Kio montros la entenon de la variablo `my_grandpa_age`. En nia kazo: `84`

## Nun, kiam vi scias tion

Nun, kiam vi konas ĉi tiun tre utilan funkcion, mi prezentas al vi la _"✨️**Lua runner**✨️"_ kaj _"✨️**Interagan Lua-redaktilon**✨️"_, per kiuj vi povas ruli viajn proprajn programojn:

<div class="runner">

```lua
print("Saluton mondo!")
```

</div>

Vi povas vidi, ke ĉi tio estas relanĉata ĉiufoje, kiam vi klakas "Run", per ĉi tiu ekzemplo

<div class="runner">

```lua
print(random(1, 100))
```

</div>

Kaj fine, ni povas certi, ke ni scias la aĝon de nia avo ĝuste:

<div class="runner">

```lua
the_best_number = 40 + 2
print(the_best_number)
my_grandpa_age = the_best_number * 2
print(my_grandpa_age)
```

</div>
