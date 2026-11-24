## Plus d'opérateurs

Vous connaissez déjà `+`, `-`, `*` et `/`. Lua a trois autres opérateurs pour les `number`s :

|Opérateur|Ce qu'il fait|
|-|-|
|`//`|Divise et supprime les décimales|
|`%`|Donne le reste d'une division|
|`^`|Élève à une puissance|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

L'opérateur `%` est très utile pour savoir si un `number` est pair ou impair : un `number` est pair lorsque le reste de sa division par `2` est `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Nombres entiers et nombres décimaux

Lua a deux sortes de `number`s : les nombres entiers comme `5`, et les nombres décimaux comme `5.0` ou `3.14`. La plupart du temps, vous n'avez pas besoin d'y penser, mais cela explique pourquoi certains résultats s'affichent avec un `.0` :

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

L'opérateur `/` donne toujours un `number` décimal, même lorsque le résultat est rond. `5` et `5.0` restent égaux.

<div class="note">

Les `number`s décimaux ne sont pas toujours exacts ! Les ordinateurs ne peuvent pas stocker parfaitement un `number` comme `0.1`, donc certaines comparaisons donnent des résultats surprenants :

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Évitez d'utiliser `==` sur le résultat d'un calcul avec des `number`s décimaux.

</div>

## Autres façons d'écrire les `number`s

Les très grands ou très petits `number`s peuvent être écrits avec `e`, qui signifie « fois 10 à la puissance ». Vous pouvez aussi écrire les `number`s en [hexadécimal](https://fr.wikipedia.org/wiki/Hexad%C3%A9cimal) en commençant par `0x` :

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Convertir les `number`s et les chaînes

La fonction `tostring` convertit un `number` en `string`, et `tonumber`, que nous avons déjà utilisé, fait l'inverse :

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua fait aussi la conversion tout seul quand il le peut : une `string` qui ressemble à un `number` devient un `number` dans un calcul, et un `number` devient une `string` avec `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` et `math.ceil`
La fonction `math.floor` arrondit un `number` vers le bas, et `math.ceil` vers le haut :

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

Pour arrondir au `number` entier le plus proche, ajoutez `0.5` avant d'utiliser `math.floor` :

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` et `math.min`
La fonction `math.max` donne le plus grand des `number`s que vous lui donnez, et `math.min` le plus petit :

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
La fonction `math.abs` retire le signe d'un `number`. C'est utile pour obtenir la distance entre deux `number`s :

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
La fonction `math.sqrt` donne la racine carrée d'un `number` :

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

Vous pouvez consulter toutes les fonctions `math` ici : <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
