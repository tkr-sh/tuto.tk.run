Une information utile dont nous avons besoin lorsque nous travaillons avec des tables est « combien d'éléments contient-elle ? ». Par exemple, si je veux obtenir le dernier élément !

Cela est possible grâce à un opérateur spécial : `#`. En écrivant `#` puis le nom de la table ou la table directement, vous pouvez obtenir la taille totale de la table :


<div class="runner">

```lua
ma_table = { "a", "b", "c" }

print(#ma_table)
ma_table[4] = "d"
print(#ma_table)
```

</div>

C'est particulièrement utile si je veux connaître l'élément d'une table avec l'index « dernier », ou si je veux ajouter un nouvel élément à la fin : `ma_table[#ma_table]` est la valeur dont la clé est la longueur totale de la table, c'est-à-dire la dernière dans la plupart des cas !

<div class="runner">

```lua
ma_table = { "a", "b", "c" }

print(ma_table[#ma_table])
ma_table[#ma_table + 1] = "d"
print(ma_table[#ma_table])
```

</div>


### Cas limites

L'opérateur `#` n'est fiable que lorsque les clés d'une table sont les nombres entiers `1, 2, 3, ...` sans trous à partir de `1`. Les clés qui sont des chaînes ou des booléens ne sont pas comptées, et si un nombre manque au milieu, le résultat est imprévisible :

<div class="runner">

```lua
-- Fiable : les clés sont 1, 2, 3
print(#{ "a", "b", "c" }) -- 3
print(#{ [1] = "a", [2] = "b", [3] = "c" }) -- 3

-- Non comptées : les clés sont des chaînes
print(#{ ["a"] = 1, ["b"] = 2, ["c"] = 3 }) -- 0

-- Non fiable : il y a un trou à l'index 2
print(#{ [1] = "a", [3] = "b", [4] = "c" }) -- 1 ou 4, ne vous y fiez pas
```

</div>
