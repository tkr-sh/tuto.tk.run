## Fonctions en ligne

Jusqu'à présent, nous écrivions les fonctions comme ceci :

```lua
function double(number)
    return number * 2
end
```

C'est du sucre. Ce que Lua fait réellement, c'est créer une fonction sans nom et la stocker dans une variable appelée `double` :

```lua
double = function(number)
    return number * 2
end
```

Les deux versions sont exactement identiques. C'est aussi pour cela que nous pouvions passer `tonumber` à `lire_validee` : une fonction n'est qu'une valeur stockée dans une variable, comme un `number` ou une `string`.


## Appeler des fonctions avec `:`

Pour mettre une chaîne en minuscules, nous écrivions `string.lower(reponse_utilisateur)`.
Mais, lorsque vous utilisez une fonction de la bibliothèque `string` ou `table`, qui prend comme premier argument une `string` ou une `table`, vous pouvez l'écrire avec la syntaxe suivante : `reponse_utilisateur:lower()` :

<div class="runner">

```lua
local reponse = "OUI"

print(string.lower(reponse))
print(reponse:lower())
```

</div>

Le `:` prend la valeur à sa gauche et la donne à la fonction comme premier argument. Pour une chaîne, Lua cherche la fonction dans la table `string`, donc `reponse:lower()` devient `string.lower(reponse)`.

<div class="note">

Cela ne fonctionne pas sur une chaîne écrite directement dans le code : `"OUI":lower()` est une erreur de syntaxe, car Lua n'accepte `:` qu'après un nom de variable ou après des parenthèses. Pour que cela fonctionne, mettez la chaîne entre parenthèses :

```lua
print(("OUI"):lower())
-- Ceci produit une erreur :
print("OUI":lower())
```

</div>

## Sans parenthèses

Lorsqu'une fonction est appelée avec un seul argument, et que cet argument est une `string` ou une `table` écrite directement dans le code, les parenthèses sont facultatives :

<div class="runner">

```lua
print "Bonjour !"
print("Bonjour !")
```

</div>

Vous verrez cela dans le code des autres. Dans ce cours, nous gardons les parenthèses, car elles rendent évident qu'une fonction est appelée.

## Renvoyer plusieurs valeurs

Une fonction peut renvoyer plus d'une valeur. Vous les séparez par `,` après le `return`, et vous les recevez dans plusieurs variables, également séparées par `,` :

<div class="runner">

```lua
function premier_et_dernier(liste)
    return liste[1], liste[#liste]
end

local premier, dernier = premier_et_dernier({ "a", "b", "c", "d" })

print(premier)
print(dernier)
```

</div>

Si vous donnez moins de variables qu'il n'y a de valeurs, les valeurs en trop sont simplement abandonnées. Si vous en donnez plus, les variables en trop valent `nil` :

<div class="runner">

```lua
function premier_et_dernier(liste)
    return liste[1], liste[#liste]
end

local seulement_premier = premier_et_dernier({ "a", "b", "c", "d" })
local premier, dernier, autre = premier_et_dernier({ "a", "b", "c", "d" })

print(seulement_premier)
print(premier, dernier, autre)
```

</div>

<div class="note">

Pour ne garder que la première valeur renvoyée par une fonction, mettez l'appel entre parenthèses :

<div class="runner">

```lua
function premier_et_dernier(liste)
    return liste[1], liste[#liste]
end

print( premier_et_dernier({ "a", "b", "c", "d" }) )
print((premier_et_dernier({ "a", "b", "c", "d" })))
```
</div>

</div>

## Arguments manquants et superflus

Lua ne vérifie pas le nombre d'arguments que vous donnez à une fonction. Un argument manquant vaut `nil`, et un argument inattendu est ignoré :

<div class="runner">

```lua
function montrer(a, b)
    print(a, b)
end

montrer(1, 2)
montrer(1)
montrer(1, 2, 3)
```

</div>

## Arguments facultatifs

Puisqu'un argument manquant vaut `nil`, nous pouvons le vérifier et utiliser une valeur par défaut à la place. Cela rend l'argument facultatif :

<div class="runner">

```lua
function saluer(nom, salutation)
    if salutation == nil then
        salutation = "Bonjour"
    end

    print(salutation .. ", " .. nom .. " !")
end

saluer("Bob")
saluer("Bob", "Bonsoir")
```

</div>
