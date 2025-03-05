Ceci est le premier programme que nous allons étudier ! Ne vous inquiétez pas, nous allons passer à travers cela TRÈS lentement, donc ce n'est pas grave si vous ne comprenez rien pour l'instant !

```lua
nombre_d_essais = 0
nombre_a_deviner = random(1, 100)

print("Essayez de deviner le nombre aléatoire entre 1 et 100 !")

repeat
	supposition_utilisateur = tonumber(read())
	nombre_d_essais = nombre_d_essais + 1

	if nombre_a_deviner < supposition_utilisateur then
		print("Le nombre est plus bas !")
	end

	if nombre_a_deviner > supposition_utilisateur then
		print("Le nombre est plus élevé !")
	end
until supposition_utilisateur == nombre_a_deviner

print("Vous avez gagné en " .. nombre_d_essais .. " essais !")
```

Ce que fait ce programme, c'est créer le jeu de "trouver le nombre" :

```
Essayez de deviner le nombre aléatoire entre 1 et 100 !
50
Le nombre est plus élevé !
75
Le nombre est plus bas !
62
Vous avez gagné en 3 essais !
```

