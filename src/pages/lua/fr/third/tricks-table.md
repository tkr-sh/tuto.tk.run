## Clés de type chaîne

Lorsqu'une clé est une `string` qui ressemble à un nom de variable, vous pouvez retirer les crochets et les guillemets :

<div class="runner">

```lua
local joueur = { ["nom"] = "Bob", ["score"] = 42 }
local meme_joueur = { nom = "Bob", score = 42 }

print(joueur["nom"])
print(meme_joueur["nom"])
```

</div>

## Accéder à une clé avec `.`

Le même raccourci existe pour lire ou modifier une valeur : `joueur.nom` est identique à `joueur["nom"]`.

<div class="runner">

```lua
local joueur = { nom = "Bob", score = 42 }

print(joueur["nom"])
print(joueur.nom)

joueur.score = 50
print(joueur.score)
```

</div>

Vous utilisez cela depuis le premier chapitre : dans `io.read()`, `io` est une `table` intégrée de Lua et `read` est l'une de ses clés !

<div class="note">

`joueur.cle` et `joueur[cle]` ne sont pas la même chose ! La première cherche la clé `"cle"`, la seconde utilise la valeur de la variable `cle` :

<div class="runner">

```lua
local joueur = { nom = "Bob", score = 42 }
local cle = "score"

print(joueur.cle) -- nil
print(joueur[cle]) -- 42
```
</div>

</div>

## `table.concat`
La fonction `table.concat` joint toutes les valeurs d'une liste en une seule `string`. Vous pouvez lui donner un séparateur à placer entre les valeurs. C'est très utile pour afficher une liste, car `print` seul ne montre pas ce qu'il y a dans une table :

<div class="runner">

```lua
local lettres = { "a", "b", "c" }

print(lettres)
print(table.concat(lettres))
print(table.concat(lettres, ", "))
```

</div>

## `table.insert`
La fonction `table.insert` ajoute une valeur à la fin d'une liste. Si vous lui donnez une position, la valeur y est insérée et les valeurs suivantes sont décalées d'un rang :

<div class="runner">

```lua
local noms = { "bob", "carol" }

table.insert(noms, "dave")
print(table.concat(noms, ", "))

table.insert(noms, 2, "alice")
print(table.concat(noms, ", "))
```

</div>

## `table.remove`
La fonction `table.remove` retire la valeur située à une position et décale les valeurs suivantes, afin qu'il n'y ait pas de trou dans la liste. Sans position, elle retire la dernière valeur. Elle renvoie aussi la valeur qui a été retirée :

<div class="runner">

```lua
local noms = { "bob", "alice", "carol" }

print(table.remove(noms, 1))
print(table.concat(noms, ", "))

print(table.remove(noms))
print(table.concat(noms, ", "))
```

</div>

## `table.sort`
La fonction `table.sort` trie une liste, de la plus petite à la plus grande valeur, ou dans l'ordre alphabétique pour les `string`s. Elle modifie la liste elle-même et n'en crée pas une nouvelle :

<div class="runner">

```lua
local nombres = { 42, 7, 19 }

table.sort(nombres)
print(table.concat(nombres, ", "))
```

</div>

Pour trier dans un autre ordre, vous pouvez lui donner une fonction. Cette fonction reçoit deux valeurs et doit renvoyer `true` si la première doit être placée avant la seconde :

<div class="runner">

```lua
local nombres = { 42, 7, 19 }

table.sort(nombres, function(a, b) return a > b end)
print(table.concat(nombres, ", "))
```

</div>

---

Vous pouvez consulter toutes les fonctions `table` ici : <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
