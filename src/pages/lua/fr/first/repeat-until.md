Voici la traduction du texte de l'introduction au langage de programmation Lua :

Pour la première fois, nous ne nous concentrerons pas sur une seule ligne ! La ligne actuelle est simplement 

```lua
repeat
```

mais en soi, elle ne fait rien : elle va avec `until`.

```lua
repeat
    supposition_utilisateur = tonumber(read())
    nombre_d_essais = nombre_d_essais + 1

    if nombre_à_trouver < supposition_utilisateur then
        imprimer("Le nombre est plus bas !")
    end

    if nombre_à_trouver > supposition_utilisateur then
        imprimer("Le nombre est plus haut !")
    end
until supposition_utilisateur == nombre_à_trouver
```

Mais avant tout... qu'est-ce que `repeat` et `until` et pourquoi sont-ils si spéciaux ? Ne sont-elles pas juste des variables étranges ?

Vous pourriez vous rappeler de la définition d'une variable

> `le_nom_de_la_variable` peut être n'importe quoi. Sauf `true`, `false`, `nil` (et d'autres que nous verrons plus tard)

Et le plus tard... est maintenant !

## Mots-clés

`repeat` et `until` sont ce que l'on appelle des "mots-clés". Les mots-clés sont des noms spéciaux qui, au lieu d'avoir une valeur (comme les variables) ou de faire une opération (comme les fonctions), sont des instructions qui disent à Lua de prendre certaines décisions.

Dans notre exemple de `repeat`/`until`, tout ce qui est entre `repeat` et `until` sera répété jusqu'à ce qu'une certaine condition soit remplie. Lorsque l'exécution arrive à `until`, elle vérifie si elle veut recommencer, et si elle le souhaite, le code revient à `repeat` et se répète.

Ce type de comportement - répéter du code jusqu'à ce que _quelque chose_ soit fait - est appelé une "boucle" et c'est un concept très courant en programmation.

La plupart du temps, vous voulez sortir de votre boucle à un certain moment...

### Mais comment savoir quand sortir ?

Lorsque vous utilisez des boucles comme `until`, vous devez fournir un booléen (`true` ou `false`) pour lui dire si vous voulez continuer la boucle.

Et la plupart du temps, vous le faites avec des opérateurs relationnels :

- `==`: Vérifiez si deux valeurs sont égales. **NE PAS CONFONDRE AVEC 1 `=`**. 1 `=` est pour attribuer une valeur, 2 `=` (`==`) est pour vérifier que 2 valeurs sont égales. Si elles sont les mêmes, la valeur est `true` sinon `false`
- `~=`: Vérifiez que deux valeurs sont différentes. Si elles sont différentes, la valeur est `true` sinon `false`
- `>`: Vérifiez que la valeur de gauche est supérieure à la valeur de droite. Si c'est le cas, la valeur est `true` sinon `false`
- `<`: Vérifiez que la valeur de gauche est inférieure à la valeur de droite. Si c'est le cas, la valeur est `true` sinon `false`
- `>=`: Vérifiez que la valeur de gauche est supérieure ou égale à la valeur de droite. Si c'est le cas, la valeur est `true` sinon `false`
- `<=`: Vérifiez que la valeur de gauche est inférieure ou égale à la valeur de droite. Si c'est le cas, la valeur est `true` sinon `false`

<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```

</div>

### Sortes de boucles

- `repeat`/`while` : va donc répéter un code jusqu'à ce qu'une certaine condition soit remplie, et s'arrêtera une fois que cette condition est remplie

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while` : Les boucles `while` sont très similaires aux boucles `repeat`/`until`, mais c'est à l'envers ! Vous faites une boucle tant que la condition est remplie ! Après la condition, vous utilisez le mot-clé `faites` pour commencer l'exécution du code, et vous revenez au début lorsque vous voyez le mot-clé `fin`. Dans la plupart des situations où vous pouvez utiliser `repeat`, vous pouvez également utiliser `while`. C'est surtout une question de goût personnel.

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for` : La dernière boucle est la boucle `for` et elle est un peu différente, car vous savez déjà combien de fois vous voulez itérer. Et vous définissez à partir de quel numéro et jusqu'à quel numéro vous voulez itérer. À la fin d'une itération, elle incrémentera automatiquement `age` de `1` !

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

## Retour à notre cas

Dans notre cas, nous voulons continuer à demander à l'utilisateur un nombre jusqu'à ce qu'il trouve le bon. Nous utilisons donc 

```lua
repeat
    ...
until supposition_utilisateur == nombre_à_trouver
```
