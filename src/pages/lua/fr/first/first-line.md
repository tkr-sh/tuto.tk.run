```lua
nombre_d_essais = 0
```

C'est la première ligne que nous voyons dans le programme.

Généralement, lorsque vous voulez comprendre ce qu'un programme fait, vous le lirez ligne par ligne, comme vous liriez un texte anglais normal, donc nous commençons ici.


## Ok cool, mais que fait cette ligne ?

Dans les langages de programmation, vous avez souvent le concept de "variable". Une variable est comme une boîte étiquetée qui contient des données. Une fois que vous créez une variable, vous pouvez lire son contenu - équivalent à ouvrir une boîte - et vous pouvez même la modifier (comme son nom l'indique: "variable") - équivalent à mettre une nouvelle valeur dans la boîte.

### Mais quelles données peut-on stocker ?

En Lua, il y a 6[^1] types de données :
- `number` (nombre) : Un nombre comme en mathématiques ! Vous pouvez déclarer un nombre en l'écrivant simplement : `123`. <br/> **Note** : pour les nombres avec des décimales (par exemple `0.5`), utilisez `.` (point) et non `,` (virgule)
- `boolean` (booléen) : Soit `true` (vrai) ou `false` (faux). (par exemple "2 est-il égal à 3 ?" => `false`).
- `nil` (nul) : Vide. Le but de `nil` est de dire que vous n'avez pas de valeur.
- `string` (chaîne de caractères) : Un groupe de lettres, de nombres ou de symboles (par exemple `Bonjour !`). Pour déclarer une `string`, vous devez mettre des guillemets (`'` ou `"`) autour du contenu (par exemple pour avoir la chaîne `Bonjour !`, vous devriez écrire `"Bonjour !"`)
- `table` : Nous verrons cela plus tard !
- `function` (fonction) : Nous verrons cela plus tard !

Lorsque vous voulez assigner des données à une variable, il suffit d'écrire :
```lua
le_nom_de_la_variable = la_valeur
```
`le_nom_de_la_variable` peut être n'importe quoi. Sauf `true`, `false`, `nil` (et d'autres que nous verrons plus tard)

`la_valeur` est un type valide tel que décrit ci-dessus.

Voici quelques exemples :
```lua
un_nombre_négatif = -1
un_gros_nombre = 100000000
le_nom_de_mon_grand_pere = "Jean"
aucune_valeur = nil
est_il_en_train_de_pleuvoir = false
```

Vous pouvez également assigner le contenu d'une variable à une autre variable :

```lua
le_nom_de_mon_grand_pere = "Jean"
le_nom_d_un_ami_de_mon_grand_pere = le_nom_de_mon_grand_pere
```

Dans ce contexte, `le_nom_de_mon_grand_pere` et `le_nom_d_un_ami_de_mon_grand_pere` seront tous les deux la chaîne `Jean`.

## Retour à notre première ligne

```lua
nombre_d_essais = 0
```

Donc, cette ligne met simplement le nombre `0` dans la variable `nombre_d_essais`. Et cela a du sens ! Parce que au début du programme, l'utilisateur n'a pas encore essayé de deviner le nombre ! Et comme le nom `nombre_d_essais` l'indique, cette variable contiendra à l'avenir le nombre d'essais que l'utilisateur a faits !

[^1] 

<div class="for-technical">

En réalité, il y a 8 types mais pour l'instant, seuls 6 sont importants :). <https://www.lua.org/pil/2.html>

</div>

