## Qu'est-ce qui compte comme `true` ?

Dans une condition, Lua n'accepte pas seulement `true` et `false` : n'importe quelle valeur peut être utilisée. La règle est simple : `false` et `nil` comptent comme faux, et **tout le reste** compte comme vrai.

Faites attention si vous connaissez un autre langage de programmation : en Lua, `0` et la chaîne vide `""` comptent comme vrai !

<div class="runner">

```lua
if 0 then
    print("0 compte comme vrai")
end

if "" then
    print("Une chaîne vide compte comme vrai")
end

if nil then
    print("Ceci n'est jamais affiché")
end
```

</div>

## Vérifier `nil` sans `== nil`

Grâce à cette règle, nous pouvons vérifier qu'une variable a une valeur en l'utilisant directement comme condition :

<div class="runner">

```lua
local nom = nil

if nom then
    print("Bonjour " .. nom)
else
    print("Il n'y a pas de nom")
end
```

</div>

`if nom then` est une façon plus courte d'écrire `if nom ~= nil then`, et `if not nom then` est une façon plus courte d'écrire `if nom == nil then`.

<div class="note">

Ce n'est pas exactement la même chose lorsque la variable peut contenir `false` : `false` n'est pas `nil`, mais il compte aussi comme faux dans une condition.

</div>

## `not`

Le mot-clé `not` donne toujours un vrai booléen. Il donne `true` pour `false` et `nil`, et `false` pour tout le reste :

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "bonjour")
```

</div>

## `or` et `and` renvoient l'une de leurs valeurs

Nous avons vu `or` et `and` avec des booléens, mais ils fonctionnent avec n'importe quelle valeur, et ils ne donnent pas toujours un booléen : ils renvoient l'une de leurs deux valeurs.

- `a or b` donne `a` s'il compte comme vrai, et `b` sinon.
- `a and b` donne `a` s'il compte comme faux, et `b` sinon.

<div class="runner">

```lua
print(nil or "défaut")
print("valeur" or "défaut")

print(nil and "suivant")
print("valeur" and "suivant")
```

</div>

Avec `true` et `false`, cela donne exactement les tables que nous avons vues auparavant.

## Valeurs par défaut avec `or`

C'est pourquoi `or` peut donner une valeur par défaut à une variable, comme nous l'avons fait pour les arguments facultatifs :

<div class="runner">

```lua
local nom = nil
nom = nom or "Invité"

print(nom)
```

</div>

Si `nom` a une valeur, elle est conservée. Si `nom` vaut `nil`, il devient `"Invité"`.

## Choisir entre deux valeurs

En utilisant `and` et `or` ensemble, nous pouvons choisir entre deux valeurs selon une condition, sur une seule ligne :

<div class="runner">

```lua
local score = 70
local resultat = score > 50 and "Vous gagnez !" or "Vous perdez !"

print(resultat)
```

</div>

C'est la même chose que :

```lua
local resultat
if score > 50 then
    resultat = "Vous gagnez !"
else
    resultat = "Vous perdez !"
end
```

<div class="note">

Cette astuce ne fonctionne pas si la première valeur est `false` ou `nil` : `condition and false or "autre"` donne toujours `"autre"`. Dans ce cas, utilisez un `if` normal.

</div>

## `and` s'arrête dès qu'il peut

Lorsque la première valeur de `and` compte comme faux, Lua ne regarde même pas la seconde. Vous vous souvenez de l'erreur que nous avons eue au début du cours en comparant `nil` avec un nombre ? `and` nous permet de l'éviter :

<div class="runner">

```lua
local supposition_utilisateur = nil

if supposition_utilisateur ~= nil and supposition_utilisateur > 10 then
    print("La supposition est plus grande que 10")
end

print("Aucune erreur !")
```

</div>

Comme `supposition_utilisateur ~= nil` vaut `false`, `supposition_utilisateur > 10` n'est jamais exécuté. De la même façon, `or` ne regarde pas sa seconde valeur lorsque la première compte comme vrai.

## `nil` dans les tables

Une clé qui n'existe pas dans une table donne `nil`, sans aucune erreur. Et pour retirer une clé d'une table, vous lui affectez `nil` :

<div class="runner">

```lua
local joueur = { nom = "Bob", score = 42 }

print(joueur.age)

joueur.score = nil
print(joueur.score)
```

</div>

<div class="note">

C'est la bonne façon de retirer une clé comme `score`, mais pas une valeur dans une liste : comme nous l'avons vu, cela créerait un trou. Pour les listes, utilisez `table.remove`.

</div>
