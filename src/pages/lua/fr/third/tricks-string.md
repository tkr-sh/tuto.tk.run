## Guillemets

Une chaîne peut être écrite avec des guillemets doubles `"..."` ou des guillemets simples `'...'`. Les deux donnent exactement la même chaîne. C'est utile lorsque votre texte contient l'un des deux :

<div class="runner">

```lua
print("C'est une belle journée")
print('Il a dit "bonjour"')
```

</div>

## Chaînes multi-lignes

Vous ne pouvez pas simplement ajouter un retour à la ligne au milieu d'une chaîne `"..."` comme ceci :

<div class="runner">

```lua
print("ligne 1
ligne 2")
```
</div>

Lua n'autorise tout simplement pas un retour à la ligne comme celui-ci dans une chaîne lorsqu'elle est écrite avec `"` ou `'`.

CEPENDANT ! Vous pouvez le faire en remplaçant `"` ou `'` par `[[`/`]]`

<div class="runner">

```lua
print([[ligne 1
ligne 2]])
```

</div>

## Longueur

L'opérateur `#` que nous avons utilisé sur les tables fonctionne aussi sur les chaînes :

<div class="runner">

```lua
print(#"bonjour")
print(#"bonjour le monde")
```

</div>

## `string.sub`
La fonction `string.sub` permet de créer une nouvelle `string`, à partir d'une partie de cette `string` en précisant l'index de début et de fin :

<div class="runner">

```lua
print(("bonjour"):sub(2, 4))
-- Identique à
-- print(string.sub("bonjour", 2, 4))
```

</div>

## `string.rep`
La fonction `string.rep` permet de répéter une `string` plusieurs fois :

<div class="runner">

```lua
print(("bonjour"):rep(3))
```

</div>

## `string.reverse`
La fonction `string.reverse`, comme son nom l'indique, inverse une `string` :

<div class="runner">

```lua
print(("bonjour"):reverse())
```

</div>

## `string.gsub`
La fonction `string.gsub` remplace une `string` par une autre, et affiche aussi combien de fois elle a été remplacée :

<div class="runner">

```lua
print(("bonjour"):gsub("jour", "soir"))
print(("bonjour"):gsub("o", "a"))
```

</div>

---

Vous pouvez consulter toutes les fonctions `string` ici : <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
