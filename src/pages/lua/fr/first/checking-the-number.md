Maintenant que nous savons comment

1. générer un nombre aléatoire
2. obtenir une saisie utilisateur
3. faire des boucles

Nous devons aider l'utilisateur en lui indiquant s'il est en dessous ou au-dessus du nombre correct.

Pour cela, nous allons utiliser un nouveau mot-clé, que vous utiliserez souvent : `if`

Ce que fait `if`, c'est essentiellement exécuter du code en fonction d'une condition. Les boucles vérifient les conditions pour faire des itérations, et `if` vérifie les conditions pour exécuter une fois.

Donc, par exemple, nous pouvons vérifier si l'utilisateur saisit `42`, et si c'est le cas, nous pouvons lui afficher un message spécial.
<div class="runner">

```lua
bon_nombre = 42

if bon_nombre == tonumber(read()) then
    print("Bon choix!")
end
```

</div>

Donc dans notre jeu

```lua
if nombre_a_trouver < supposition_utilisateur then
    print("Le nombre est plus bas!")
end

if nombre_a_trouver > supposition_utilisateur then
    print("Le nombre est plus haut!")
end
```

nous vérifions si le nombre est plus bas/plus haut, et en fonction de cela, nous affichons un message pour aider l'utilisateur.

Si l'utilisateur trouve le nombre correct, il arrêtera la boucle à cause de :

```lua
until supposition_utilisateur == nombre_a_trouver
```

<div class="runner">

```lua
nombre_a_trouver = 42
supposition_utilisateur = tonumber(read())

if nombre_a_trouver < supposition_utilisateur then
    print("Le nombre est plus bas!")
end

if nombre_a_trouver > supposition_utilisateur then
    print("Le nombre est plus haut!")
end

if nombre_a_trouver == supposition_utilisateur then
    print("Correct!")
end
```

</div>

Et avec cela fait, nous n'avons plus qu'à faire une bonne fin... 
