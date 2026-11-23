Une autre clé pour travailler avec les tables est de les parcourir. Disons que nous avons, dans notre jeu, la table `scores` (un `score` étant le `nombre_d_essais` qu'un utilisateur a mis pour trouver le nombre). Comment afficher chaque score pour chaque partie ?

## `for` avec index
D'abord, vous vous souvenez peut-être du mot-clé `for` dont nous avons parlé rapidement auparavant :

<blockquote>

`for` : la dernière boucle est la boucle `for`, et elle est un peu différente, car vous savez déjà combien de fois vous voulez itérer. Vous définissez de quel nombre à quel nombre vous voulez itérer. À la fin d'une itération, elle incrémentera automatiquement `age` de `1` !

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

Si nous relions cela à notre nouvelle connaissance de l'opérateur `#`, nous pouvons donc parcourir une table avec succès !

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for cle = 1, #scores do
    print(cle .. ": " .. scores[cle])
end
```
</div>

Cela fonctionne très bien ! Mais il peut être un peu redondant d'écrire toujours `scores[cle]` pour accéder à la valeur, surtout si vous ne vous souciez pas de la clé, ou si vous ne prévoyez pas de réutiliser la table. Heureusement, Lua propose une autre façon de faire.

## `ipairs`

`ipairs` est une fonction que nous allons utiliser avec le mot-clé `for` pour accéder rapidement à la clé et à la valeur sans avoir à référencer le nom de la table ! Sa syntaxe est un peu différente de la boucle `for` précédente, même si elle utilise toujours le mot-clé `for` :

```lua
for ma_cle, ma_valeur in ipairs(ma_table) do 
    -- Faire quelque chose !
end
```

Dans cet exemple, nous supposons que nous avons une `table` appelée `ma_table`. Chaque ligne de code comprise entre le `for` et le `end` s'exécutera avec `ma_cle` et `ma_valeur` étant la clé et la valeur associée de `ma_table`.

Comme la boucle `for` précédente, elle est triée par ordre croissant. C'est-à-dire qu'elle commence à `1` et augmente de `1` à chaque itération. Mais elle ne fonctionne pas lorsque les clés sont des `string`s ou s'il y a des trous entre les nombres. Si c'est notre cas, nous devons utiliser `pairs`.

## `pairs`

`pairs` est comme `ipairs`, à la différence qu'il fonctionne avec des clés qui ne sont pas des entiers consécutifs à partir de `1`. L'inconvénient est que l'ordre dans lequel les clés arrivent est imprévisible :

<div class="runner">

```lua
ma_table = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for cle, valeur in pairs(ma_table) do
    print(cle)
    print(valeur)
end
```

</div>

L'ordre dans lequel les clés/valeurs sont affichées est imprévisible et peut varier d'une exécution à l'autre. Cependant, il a l'avantage de nous permettre de parcourir n'importe quelle table !

## Retour à notre jeu
Dans notre cas, nous voulons simplement conserver le `nombre_d_essais` pour chaque partie. Donc, pas besoin de clés personnalisées, nous pouvons simplement avoir des index commençant à `1` et augmentant de `1` à chaque fois, correspondant au numéro de la partie !
