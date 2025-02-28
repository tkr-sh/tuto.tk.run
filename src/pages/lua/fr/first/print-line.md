```lua
print("Essaye de deviner le nombre aléatoire entre 1 et 100 !")
```

Cette ligne est également un appel de fonction ! On peut le voir puisqu'on a le nom `print` mais avec des parenthèses après son nom !

Cette fonction prend une chaîne de caractères comme premier et seul paramètre. Mais... on ne met jamais la sortie dans une variable.

## Pourquoi ?

Parce que `print` ne renvoie pas de valeur. `print` est une fonction spéciale qui affiche à l'utilisateur les données qu'elle reçoit en entrée.

Par exemple, reprenons notre exemple précédent :

```lua
le_meilleur_nombre = 40 + 2
l_age_de_mon_grand_pere = le_meilleur_nombre * 2
```

Comment sais-je quelle est la valeur de `l_age_de_mon_grand_pere` ? Je pourrais deviner en faisant le calcul dans ma tête, mais lorsque le programme devient plus complexe, ce n'est pas toujours possible.

Par conséquent, on peut simplement faire

```lua
print(l_age_de_mon_grand_pere)
```

Ce qui affichera le contenu de la variable `l_age_de_mon_grand_pere`. Dans notre cas : `84`.

## Maintenant que vous savez cela

Maintenant que vous savez cette fonction très utile, je vous présente l'_"**✨️Exécuteur Lua✨️**"_ et l'_"**✨️Éditeur Lua interactif✨️**"_ sur lequel vous pouvez exécuter vos propres programmes :

<div class="runner">

```lua
print("Bonjour le monde !")
```

</div>

Vous pouvez voir que cela se ré-exécute chaque fois que vous cliquez sur "Exécuter" avec cet exemple

<div class="runner">

```lua
print(random(1, 100))
```

</div>

Et enfin, on peut être sûr de connaître correctement l'âge de notre grand-père :

<div class="runner">

```lua
le_meilleur_nombre = 40 + 2
print(le_meilleur_nombre)
l_age_de_mon_grand_pere = le_meilleur_nombre * 2
print(l_age_de_mon_grand_pere)
```

</div>
