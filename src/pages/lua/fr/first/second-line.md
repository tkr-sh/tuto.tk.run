```lua
nombre_a_deviner = random(1, 100)
```

Ok, on vient de voir que vous pouvez mettre des données dans une variable... mais qu'est-ce que c'est que ça ???

Pour comprendre ce que cela fait, nous devons comprendre les `function`s. C'était l'un des types de données que je n'ai pas expliqué lors de la première ligne, alors voici :

Vous pouvez voir les fonctions comme une sorte de machine qui prend une entrée, fait quelque chose avec elle, et puis vous donne une sortie.

## Mais que fait cette machine ??

Eh bien, dans Lua, il y a des opérateurs qui vous permettent de faire des choses spéciales :

- `+`: Additionne 2 nombres. (par exemple : `1 + 2` => `3`)
- `-`: Soustrait un nombre d'un autre. (par exemple : `3 - 2` => `1`)
- `*`: Multiplie 2 nombres. (par exemple : `3 * 2` => `6`)
- `/`: Divise un nombre par un autre. (par exemple : `6 / 2` => `3`)

Par conséquent, nous pouvons faire les choses suivantes avec nos variables
```lua
le_meilleur_nombre = 40 + 2
l_age_de_mon_grand_pere = le_meilleur_nombre * 2
```

`le_meilleur_nombre` sera 42 et `l_age_de_mon_grand_pere` sera 84.

## Retour à nos fonctions !

Par exemple, une fonction pourrait prendre un nombre et vous retourner le nombre `+ 1`.

Une fonction, comme une variable, peut être utilisée lorsque vous connaissez son nom. MAIS ! Lorsque vous voulez qu'elle fasse sa "machinerie" et vous donne la sortie, vous devez mettre des parenthèses après le nom (par exemple : `ma_fonction()`). Cela exécutera une fonction appelée `ma_fonction`. Mais cette fonction n'a pas d'entrée ! Pour ajouter une entrée à celle-ci, vous pouvez mettre les valeurs que vous voulez entre les parenthèses et les séparer par `,`.

### Exemple

```lua
jour = 4
mois = "avril"
annee = 1984
calculer_la_date_de_demain(jour, mois, annee)
```
ce qui est équivalent à
```lua
calculer_la_date_de_demain(4, "avril", 1984)
```

## Retour à notre deuxième ligne

```lua
nombre_a_deviner = random(1, 100)
```

Donc, cette ligne exécute la fonction `random` avec comme entrée : `1` et `100`, et met la sortie du calcul dans `nombre_a_deviner`. Nous n'avons jamais défini ce que faisait `random`. Mais Lua vient avec des fonctions "intégrées" pour aider les développeurs ! Et dans notre cas, il y a une fonction `random` qui veut 2 nombres en entrée et qui génère un nombre aléatoire entre le premier nombre (dans notre cas `1`) et le deuxième nombre (dans notre cas `100`).

Parfait ! Nous venons de générer le nombre que l'utilisateur doit deviner !

