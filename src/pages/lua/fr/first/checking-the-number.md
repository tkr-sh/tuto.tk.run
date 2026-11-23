Maintenant que nous savons comment

1. générer un nombre aléatoire
2. obtenir une saisie utilisateur
3. faire des boucles

Nous devons aider l'utilisateur en lui indiquant si son nombre est plus petit ou plus grand que le nombre à deviner.

Pour cela, nous allons utiliser un nouveau mot-clé que vous utiliserez souvent : `if`

Ce que fait `if`, c'est essentiellement exécuter du code en fonction d'une condition. Les boucles vérifient une condition pour continuer à boucler, et `if` vérifie une condition pour exécuter du code une seule fois.

Par exemple, nous pouvons vérifier si l'utilisateur saisit `42`, et si c'est le cas, nous pouvons lui afficher un message spécial.
<div class="runner">

```lua
bon_nombre = 42

if bon_nombre == tonumber(read()) then
    print("Bon choix !")
end
```

</div>

Donc dans notre jeu

```lua
if nombre_a_deviner < supposition_utilisateur then
    print("Le nombre est plus bas !")
end

if nombre_a_deviner > supposition_utilisateur then
    print("Le nombre est plus haut !")
end
```

nous vérifions si le nombre est plus bas ou plus haut, et en fonction de cela, nous affichons un message pour aider l'utilisateur.

Si l'utilisateur trouve le bon nombre, la boucle s'arrêtera grâce à :

```lua
until supposition_utilisateur == nombre_a_deviner
```

<div class="runner">

```lua
nombre_a_deviner = 42
supposition_utilisateur = tonumber(read())

if nombre_a_deviner < supposition_utilisateur then
    print("Le nombre est plus bas !")
end

if nombre_a_deviner > supposition_utilisateur then
    print("Le nombre est plus haut !")
end

if nombre_a_deviner == supposition_utilisateur then
    print("Correct !")
end
```

</div>

Et une fois cela fait, il ne nous reste plus qu'à écrire une belle fin...
