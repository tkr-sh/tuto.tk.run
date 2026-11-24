```lua
user_guess = tonumber(read())
```

Bone, do ĉi tiu kodo eble ŝajnas stranga (funkcio en funkcio?), sed ĝi povas esti vidata kiel:

```lua
read_value = read()
user_guess = tonumber(read_value)
```

Kio okazas estas la jeno:
1. Lua volas komputi la ĝustan valoron 
2. Lua volas plenumi `tonumber`, sed por tio ĝi bezonas, ke ĉiuj ĝiaj enigoj havu valorojn
3. Tial, kiam ĝi vidas `read()`, ĝi tuj provas ruli ĝin, por ke la funkcio produktu la valoron
4. Post kiam `read()` ruliĝis kaj redonis valoron, `tonumber` nun povas esti plenumata!

### Bone, sed kion faras `read()`?

`read()` petas ĉenon de la uzanto kaj kaptas ĝin.

**Antaŭ ol ruli la ekzemplon:** ne malŝaltu la ŝprucfenestrojn por ĉi tiu retejo, alie vi ne plu povos uzi `read()`!



<div class="runner">

```lua
print(read())
```

</div>

### Kaj kio pri `tonumber`?

Ĉar `read()` ricevas `string` de la uzanto, ni ne povas kompari ĝin kun nia hazarda nombro, ĉar vi ne povas kompari `string`, kiu povus enhavi ne-ciferajn signojn, kun `number` (ekz. `"x4%$A" > 3` ne havas sencon)

Tial, ni devas konverti la enigon de la uzanto al nombro, kaj ĝuste tion faras `tonumber`!

Ĉi tiu funkcio redonos `nil` se ĝia enigo ne estas valida nombro.

<div class="runner">

```lua
print(tonumber("Ne nombro") == nil)
```

</div>

Kaj vi ne povas adicii `nil` + `number`, do enigu validan nombron!

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Legos la `string` de la uzanto, konvertos ĝin al `number` kaj aldonos unu al ĉi tiu nombro, kaj se ĝi ne estas valida nombro, `tonumber(read())` estos `nil`-valoro.
