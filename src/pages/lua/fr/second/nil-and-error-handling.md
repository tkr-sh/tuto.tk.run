La première chose que vous avez peut-être remarquée, c'est que notre jeu *plante* actuellement lorsque nous saisissons un nombre qui n'est pas un nombre valide :

<div class="runner">

```lua
supposition_utilisateur = tonumber(io.read())

if 10 < supposition_utilisateur then
	print("Le nombre est plus bas !")
end
```

</div>

ce qui, si vous ne tapez pas un nombre, vous donnera :

```
Error: [string "supposition_utilisateur = tonumber(io.read())..."]:3: attempt to compare number with nil
```

Mais qu'est-ce que cela signifie ?

Le concept central ici est `nil`. En présentant les types Lua, nous avons brièvement vu le type `nil` :

> `nil` : Vide. Le but de `nil` est de dire que vous n'avez pas de valeur.

<div style="height: 3px"></div>

Et c'est _exactement_ ce que fait `tonumber` :

- Si `io.read()` est une chaîne qui représente un nombre comme `"3.14"` ou `"42"`, Lua sait la convertir d'un type `string` vers un type `number`.
- Sinon, si `io.read()` est une chaîne qui ne ressemble pas à un `number`, `tonumber` dit simplement « je ne sais pas convertir ceci en nombre, renvoyons `nil` ».

Par conséquent, si dans `io.read()` nous ne passons pas une représentation correcte d'un nombre, nous aurons simplement `nil` :

<div class="runner">

```lua
supposition_utilisateur = tonumber("123abc")
print(supposition_utilisateur)
```

</div>

## Ok, mais pourquoi cela crée-t-il une erreur alors ?

Bonne question ! Pour l'instant, `supposition_utilisateur` vaut simplement `nil`. L'erreur se produit lorsque nous essayons de comparer avec l'opérateur `<` un `number` à `nil` avec cette ligne :

```lua
if 10 < supposition_utilisateur then
```

Et l'erreur est bien décrite dans le message d'erreur :

```
Error: [string "supposition_utilisateur = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` indique la ligne à laquelle l'erreur se produit, dans ce cas, notre ligne de comparaison.
- `attempt to compare number with nil` est la raison pour laquelle il y a eu une erreur.

Et cela a parfaitement du sens ! Que signifie comparer, en termes de supérieur ou inférieur, rien avec un nombre ? Est-ce `false` ? Est-ce `true` ? Non, la question n'a tout simplement aucun sens. Donc Lua renvoie une erreur.

## Gérer l'erreur
La façon la plus rapide de gérer cela est simplement de vérifier si `supposition_utilisateur` est `nil`, ce qui peut être fait avec l'opérateur `==` :

```lua
if supposition_utilisateur == nil then
	print("Ce n'est pas un nombre valide")
end
```

Mais dans ce cas, nous ne voulons pas simplement afficher quelque chose et continuer la boucle, car cela compterait comme une tentative à cause de :

```lua
nombre_d_essais = nombre_d_essais + 1
```

Une façon simple de faire cela serait de demander `supposition_utilisateur` jusqu'à ce qu'il ne soit plus `nil` avec l'opérateur `~=`, c'est-à-dire jusqu'à ce qu'il soit un `number` valide :

```lua
repeat
	supposition_utilisateur = tonumber(io.read())
until supposition_utilisateur ~= nil
```

On peut aussi ajouter un message d'erreur au cas où, avec :

```lua
repeat
	supposition_utilisateur = tonumber(io.read())
	if supposition_utilisateur == nil then 
		print("Ce n'est pas un nombre valide !")
	end
until supposition_utilisateur ~= nil
```

Ce qui nous donne ce programme complet :


<div class="runner">

```lua
nombre_d_essais = 0
nombre_a_deviner = math.random(1, 100)

print("Essayez de deviner le nombre aléatoire entre 1 et 100 !")

repeat
	repeat
		supposition_utilisateur = tonumber(io.read())
		if supposition_utilisateur == nil then 
			print("Ce n'est pas un nombre valide !")
		end
	until supposition_utilisateur ~= nil
	nombre_d_essais = nombre_d_essais + 1

	if nombre_a_deviner < supposition_utilisateur then
		print("Le nombre est plus bas !")
	end

	if nombre_a_deviner > supposition_utilisateur then
		print("Le nombre est plus haut !")
	end
until supposition_utilisateur == nombre_a_deviner

print("Vous avez gagné en " .. nombre_d_essais .. " essais !")
```

</div>
