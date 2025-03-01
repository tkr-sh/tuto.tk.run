Donc, maintenant, la seule chose qui reste est d'imprimer le nombre de tentatives.

Nous avions précédemment incrémenté ce dernier juste après avoir récupéré les données de l'utilisateur avec 

```lua
nombre_d_essais = nombre_d_essais + 1
```

Mais maintenant, nous aimerions imprimer un message de félicitations et en même temps, combien de tentatives cela a-t-il pris ?

Il est temps d'apprendre l'opérateur de concaténation avant la fin de ce premier jeu :

`..` : Concatène 2 valeurs.

<div class="runner">

```lua
print("Cela fait " .. 4 .. " ans")
```

</div>

Avec cela en tête, nous pouvons créer un message sympa :

```lua
print("Vous avez gagné en " .. nombre_d_essais .. " tentatives !")
```

Et voici ENFIN notre jeu :

---

<div class="runner">

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
        print("Le nombre est plus haut !")
    end
until supposition_utilisateur == nombre_a_deviner

print("Vous avez gagné en " .. nombre_d_essais .. " essais !")
```

</div>

---

## WOW.

Wow ! N'est-ce pas ?

Vous êtes prêt à devenir programmeur maintenant :) !

## Est-ce fini ?

Oui et non.

Hélas, j'ai simplifié une chose : en général, vous ne pouvez pas exécuter `read` et `random` comme cela. Lua aime mettre les fonctions "intégrées" dans certaines catégories : `math` (pour les fonctions mathématiques), `io` (pour les interactions entrée/sortie), `os` (pour interagir avec votre système d'exploitation), ...

Et par conséquent, lorsque vous voulez utiliser des fonctions spécifiques qui font partie d'une catégorie, vous devez les appeler comme `catégorie.fonction()`

Dans notre cas, `random` fait partie de la catégorie `math`, et `read` fait partie de la catégorie `io`. `print` et `tonumber` sont globales, donc vous n'avez pas besoin de spécifier quoi que ce soit.

<div class="runner">

```lua
nombre_d_essais = 0
nombre_a_deviner = math.random(1, 100)

print("Essayez de deviner le nombre aléatoire entre 1 et 100 !")

repeat
    supposition_utilisateur = tonumber(io.read())
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

## Félicitations, vous avez maintenant terminé votre premier programme Lua ! 🎉

Maintenant que vous avez les bases, nous allons essayer d'améliorer notre jeu actuel pour ajouter de nouvelles fonctionnalités, et nous allons apprendre de nouvelles choses en route !

