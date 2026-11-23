Et si nous pouvions avoir plusieurs parties dans une seule exécution, et laisser l'utilisateur décider quand s'arrêter ?

Pour rendre le jeu répétable, nous enveloppons simplement le programme dans une autre boucle `repeat ... until` comme ceci :

```lua
repeat
	...
	print("Vous avez gagné en ... essais !")

	repeat 
		print("Voulez-vous continuer ? oui / non")
		reponse_utilisateur = io.read()
	until reponse_utilisateur == "oui" or reponse_utilisateur == "non"
until reponse_utilisateur == "non"

print("Au revoir !")
```

Nous utilisons un mot-clé spécial `or`, qui combine deux conditions et qui vaut `true` si au moins l'une d'elles est vraie.

Prenons par exemple un booléen `a` qui peut être `true` ou `false`, et un autre booléen `b` qui peut aussi être `true` ou `false`. L'expression `a or b` sera donc :

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` n'est `false` que lorsque `a` et `b` sont `false`.
C'est l'inverse de l'opérateur `and`, qui exige que `a` et `b` soient `true` pour valoir `true`.

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## La casse

En Lua, une chaîne contenant des lettres majuscules comme `"A"`, `"B"`, `"C"`, ... n'est pas la même que `"a"`, `"b"`, `"c"`, .... Cela signifie que si un utilisateur tape `"Oui"`, cela ne fonctionnera pas et l'utilisateur refera la boucle `repeat` alors même qu'il a tapé `"Oui"`. Ce n'est probablement pas ce que nous voulons...

Heureusement, Lua propose des fonctions qui nous permettent de convertir une chaîne en majuscules ou en minuscules : `string.upper` et `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

Ce qui nous permet de faire ce qui suit

<div class="runner">

```lua
repeat 
	print("Voulez-vous continuer ? oui / non")
	reponse_utilisateur = string.lower(io.read())
	if reponse_utilisateur ~= "oui" and reponse_utilisateur ~= "non" then
		print("Ce n'est pas une réponse valide.")
	end
until reponse_utilisateur == "oui" or reponse_utilisateur == "non"
```

</div>

Ainsi, `reponse_utilisateur` sera toujours en minuscules grâce à `string.lower()`.
Nous ajoutons aussi une vérification pour afficher un message si la réponse n'est ni `"oui"` ni `"non"`. 
Ce qui nous donne ce programme complet :

<div class="runner">

```lua
repeat 
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

	repeat 
		print("Voulez-vous continuer ? oui / non")
		reponse_utilisateur = string.lower(io.read())
		if reponse_utilisateur ~= "oui" and reponse_utilisateur ~= "non" then
			print("Ce n'est pas une réponse valide.")
		end
	until reponse_utilisateur == "oui" or reponse_utilisateur == "non"
until reponse_utilisateur == "non"

print("Au revoir !")
```

</div>
