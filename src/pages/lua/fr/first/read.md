```lua
supposition_utilisateur = tonumber(read())
```

Ok, ce code peut sembler étrange (une fonction dans une autre?), mais il peut être vu comme suit :

```lua
valeur_lue = read()
supposition_utilisateur = tonumber(valeur_lue)
```

Ce qui se passe est le suivant :
1. Lua veut calculer la valeur de droite 
2. Lua veut exécuter `tonumber`, mais pour cela, il lui faut que toutes ses entrées aient des valeurs
3. Par conséquent, lorsqu'il voit `read()`, il tente immédiatement de l'exécuter afin que la fonction produise une valeur
4. Après que `read()` ait été exécuté et ait retourné une valeur, `tonumber` peut maintenant être exécuté !

### Ok, mais qu'est-ce que `read()` fait ?

`read()` demande à l'utilisateur de saisir une chaîne de caractères et la capture.

**Avant d'exécuter l'exemple :** ne désactivez pas les fenêtres contextuelles pour ce site, sinon vous ne pourrez plus utiliser `read()` !

<div class="runner">

```lua
print(read())
```

</div>

### Et qu'en est-il de `tonumber` ?

Puisque `read()` obtient une chaîne de caractères de l'utilisateur, nous ne pouvons pas la comparer à notre nombre aléatoire, car il n'est pas possible de comparer une chaîne de caractères qui peut contenir des caractères non numériques à un nombre (par exemple, `"x4%$A" > 3` n'a pas de sens)

Par conséquent, nous devons convertir la saisie de l'utilisateur en un nombre et c'est exactement ce que fait `tonumber` !

Cette fonction retournera `nil` si son entrée n'est pas un nombre valide.

<div class="runner">

```lua
print(tonumber("Pas un nombre") == nil)
```

</div>

Et vous ne pouvez pas additionner `nil` + `nombre`, donc saisissez un nombre valide !

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Lira la chaîne de caractères saisie par l'utilisateur, la convertira en nombre et ajoutera un à ce nombre. Si ce n'est pas un nombre valide, `tonumber(read())` sera une valeur `nil`. 
