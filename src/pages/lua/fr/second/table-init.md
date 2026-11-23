Et si maintenant nous pouvions avoir un historique de nos scores, jusqu'à ce que nous ayons fini de jouer ? Un historique de tout ce qui s'est produit ? Comment ferions-nous ?

Votre première approche pourrait être de définir plusieurs variables comme ceci :

```lua 
score_premiere_partie = nil
score_deuxieme_partie = nil
score_troisieme_partie = nil
...
```

Mais ce n'est pas une bonne approche. Ce n'est pas viable si l'utilisateur joue beaucoup de fois. C'est difficile à relier proprement à une boucle.

Pour résoudre cela, nous aurons besoin d'un nouveau type de données : `table`.

## Qu'est-ce qu'une table ?

Une `table` est un type Lua qui nous permet de stocker une quantité de données arbitrairement longue ou courte. Chaque table Lua associe une clé à une valeur au format `[cle] = valeur`, comme ceci :

```lua
ma_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

Nous pouvons accéder à la valeur d'une table en référençant la clé et en écrivant `ma_table[ma_cle]`. Par exemple :

<div class="runner">

```lua
ma_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(ma_table["b"])
```

</div>

Et nous pouvons simplement ajouter une nouvelle valeur à notre table en associant une clé inexistante à une valeur, comme ceci :

<div class="runner">

```lua
ma_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

ma_table["d"] = 5
print(ma_table["d"])
```

</div>

Vous pouvez aussi ne pas écrire de clé, et dans ce cas, les clés commenceront à un et augmenteront ensuite :


<div class="runner">

```lua
ma_table = { "a", "b", "c", "z" }
-- Identique à
-- ma_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(ma_table[2])
print(ma_table[4])
```

</div>
