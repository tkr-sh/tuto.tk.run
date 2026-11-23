Pour la première fois, nous n'allons pas nous concentrer sur une seule ligne ! La ligne actuelle est simplement

```lua
repeat
```

mais en soi, elle ne fait rien : elle va de pair avec `until`.

```lua
repeat
	supposition_utilisateur = tonumber(read())
	nombre_d_essais = nombre_d_essais + 1

	if nombre_a_deviner < supposition_utilisateur then
		print("Le nombre est plus bas !")
	end

	if nombre_a_deviner > supposition_utilisateur then
		print("Le nombre est plus haut !")
	end
until supposition_utilisateur == nombre_a_deviner
```

Mais avant tout... que sont `repeat` et `until`, et pourquoi sont-ils si spéciaux ? Ne sont-ils pas simplement des variables étranges ?

Vous vous rappelez peut-être la définition d'une variable

> `le_nom_de_la_variable` peut être n'importe quoi. Sauf `true`, `false`, `nil` (et d'autres que nous verrons plus tard)

Et ce « plus tard »... c'est maintenant !

## Mots-clés

`repeat` et `until` sont ce que l'on appelle des « mots-clés ». Les mots-clés sont des noms spéciaux qui, au lieu d'avoir une valeur (comme les variables) ou d'effectuer un calcul (comme les fonctions), sont des instructions qui disent à Lua de prendre certaines décisions.

Dans notre exemple avec `repeat`/`until`, tout ce qui se trouve entre `repeat` et `until` sera répété jusqu'à ce qu'une certaine condition soit remplie. Lorsque l'exécution arrive à `until`, elle vérifie si elle doit recommencer, et si oui, le code revient à `repeat`, et se répète.

Ce type de comportement - répéter du code jusqu'à ce que _quelque chose_ soit vrai - est appelé une « boucle », et c'est un concept très courant en programmation.

La plupart du temps, vous voulez sortir de votre boucle à un moment donné...

### Mais comment savoir quand sortir ?

Lorsque vous utilisez des boucles comme `until`, vous devez leur fournir un `boolean` (`true` ou `false`) pour leur indiquer quand arrêter de boucler.

Et la plupart du temps, vous le faites avec des opérateurs relationnels :

- `==` : Vérifie si deux valeurs sont égales. **À NE PAS CONFONDRE AVEC 1 `=`**. 1 `=` sert à assigner une valeur, 2 `=` (`==`) servent à vérifier que 2 valeurs sont égales. Si elles sont identiques, la valeur est `true`, sinon `false`
- `~=` : Vérifie que deux valeurs sont différentes. Si elles sont différentes, la valeur est `true`, sinon `false`
- `>` : Vérifie que la valeur de gauche est supérieure à celle de droite. Si c'est le cas, la valeur est `true`, sinon `false`
- `<` : Vérifie que la valeur de gauche est inférieure à celle de droite. Si c'est le cas, la valeur est `true`, sinon `false`
- `>=` : Vérifie que la valeur de gauche est supérieure **OU ÉGALE** à celle de droite. Si c'est le cas, la valeur est `true`, sinon `false`
- `<=` : Vérifie que la valeur de gauche est inférieure **OU ÉGALE** à celle de droite. Si c'est le cas, la valeur est `true`, sinon `false`


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

### Types de boucles

- `repeat`/`until` : va donc répéter du code jusqu'à ce qu'une certaine condition soit remplie, et s'arrêter une fois cette condition remplie

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while` : les boucles `while` sont très similaires aux boucles `repeat`/`until`, mais dans l'autre sens ! Vous bouclez tant que la condition est remplie ! Après la condition, vous utilisez le mot-clé `do` pour commencer à exécuter le code, et vous revenez au début lorsque vous rencontrez le mot-clé `end`. Dans la plupart des situations où vous pouvez utiliser `repeat`, vous pouvez aussi utiliser `while`. C'est surtout une question de goût personnel.

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for` : la dernière boucle est la boucle `for`, et elle est un peu différente, car vous savez déjà combien de fois vous voulez itérer. Vous définissez de quel nombre à quel nombre vous voulez itérer. À la fin d'une itération, elle incrémentera automatiquement `age` de `1` !

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>


## Retour à notre cas

Dans notre cas, nous voulons continuer à demander un nombre à l'utilisateur jusqu'à ce qu'il trouve le bon. Nous écrivons donc
```lua
repeat
    ...
until supposition_utilisateur == nombre_a_deviner
```
