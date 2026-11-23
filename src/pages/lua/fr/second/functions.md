Vous avez peut-être remarqué que nous répétons un bloc de code similaire deux fois pour la validation des entrées :

```lua
repeat
    print("Essayez de deviner un nombre :")
    supposition_utilisateur = tonumber(io.read())
    if supposition_utilisateur == nil then 
        print("Ce n'est pas un nombre valide !")
    end
until supposition_utilisateur ~= nil
```

```lua
repeat 
    print("Voulez-vous continuer ? oui / non")
    reponse_utilisateur = string.lower(io.read())
    if reponse_utilisateur ~= "oui" and reponse_utilisateur ~= "non" then
        print("Ce n'est pas une réponse valide.")
    end
until reponse_utilisateur == "oui" or reponse_utilisateur == "non"
```

Il y a clairement un motif commun dupliqué que je vais tenter d'illustrer avec une syntaxe pseudo/invalide délimitée par `<` et `>`.

```lua
repeat 
    print(<question>)
    saisie_utilisateur = <transformer>(io.read())
    if <condition_inverse> then
        print(<message_erreur>)
    end
until <condition>
```

## Les bases d'une fonction

Lorsque vous écrivez un programme, il y a souvent une règle à suivre : écrire les choses de façon modulaire, pour qu'elles soient utiles à l'avenir et assez génériques pour gérer le plus de cas possibles, et c'est exactement ce que les fonctions nous aident à accomplir.

Une fonction est essentiellement un ensemble d'instructions qu'un programme exécutera. Une fonction peut prendre des paramètres et renvoyer une valeur. Prenons cette fonction par exemple :

```lua
function double(mon_nombre)
    return mon_nombre * 2
end
```

Le nom de cette fonction est `double` et elle prend un seul paramètre appelé `mon_nombre`. Le résultat de cette fonction, ce qu'elle renvoie, est une seule valeur : `mon_nombre * 2`. Vous pouvez appeler une fonction en spécifiant son nom puis, entre parenthèses et séparés par `,`, les arguments, comme ceci `ma_fonction(valeur1, valeur2)`. Dans notre cas :

<div class="runner">

```lua
function double(mon_nombre)
    return mon_nombre * 2
end

print(double(42))
```

</div>

## Retour à notre jeu !

Optimisons donc la gestion des entrées ! La première chose à examiner est : de quoi dépend notre fonction ? C'est nécessaire pour savoir de quels arguments elle a besoin. Dans notre cas :
* Message d'invite, le message qui dit à l'utilisateur de saisir une valeur
* Message d'erreur, le message à afficher lorsque l'entrée n'est pas valide
* Transformation, une transformation que nous appliquons à la chaîne lue auprès de l'utilisateur (`tonumber`, `string.lower`, `string.upper`, ...)
* Condition, une vérification pour savoir si l'entrée correspond à ce que nous attendons après avoir appliqué la transformation.

`message_invite` et `message_erreur` sont assez simples, ce sont juste des chaînes.<br/>

`transformer` et `condition` sont un peu plus compliqués.<br>
`transformer` prend l'entrée de l'utilisateur et renvoie une version modifiée de celle-ci. C'est clairement le cas d'une fonction : quelque chose qui prend une entrée et renvoie quelque chose.<br/>
`condition` est également très similaire : elle prend en argument le résultat de `transformer` et renvoie un booléen (`true`/`false`) selon le résultat. C'est encore clairement un cas où une `function` est nécessaire.

```lua
function lire_validee(message_invite, message_erreur, transformer, condition)
    repeat 
        print(message_invite)
        saisie_utilisateur = transformer(io.read())
        est_valide = condition(saisie_utilisateur)

        if not est_valide then
            print(message_erreur)
        end
    until est_valide

    return saisie_utilisateur
end
```

Remarquez le `not` ! Le mot-clé `not` inverse un booléen : `true` => `false`, `false` => `true`.

Dans notre cas, nous pouvons simplement écrire ceci pour la gestion de la saisie du nombre par l'utilisateur :

```lua
function est_non_nil(arg) 
    return arg ~= nil
end

lire_validee(
    "Essayez de deviner un nombre :",
    "Ce n'est pas un nombre valide",
    tonumber,
    est_non_nil
)
```

Cela produira la même chose que :

```lua
repeat
    print("Essayez de deviner un nombre :")
    supposition_utilisateur = tonumber(io.read())
    est_valide = supposition_utilisateur ~= nil
    if not est_valide then 
        print("Ce n'est pas un nombre valide !")
    end
until est_valide
```

## Fonctions en ligne

Remarquez que nous avons défini `est_non_nil` uniquement pour l'utiliser une seule fois. Au lieu de déclarer une fonction avec un nom quand nous n'en avons pas besoin, Lua nous permet de simplement retirer le nom lorsque nous ne voulons pas l'utiliser ailleurs, comme ceci :

```lua
local double = function(nombre)
    return nombre * 2
end
```

Dans notre cas, nous pouvons l'utiliser pour faire :

```lua
lire_validee(
    "Essayez de deviner un nombre :",
    "Ce n'est pas un nombre valide",
    tonumber,
    function(nb) return nb ~= nil end
)
```

Ce qui nous donne notre programme final :

<div class="runner">

```lua
function lire_validee(message_invite, message_erreur, transformer, condition)
    repeat 
        print(message_invite)
        saisie_utilisateur = transformer(io.read())
        est_valide = condition(saisie_utilisateur)

        if not est_valide then
            print(message_erreur)
        end
    until est_valide

    return saisie_utilisateur
end

repeat 
	nombre_d_essais = 0
	nombre_a_deviner = math.random(1, 100)

	print("Essayez de deviner le nombre aléatoire entre 1 et 100 !")

	repeat
        supposition_utilisateur = lire_validee(
            "Entrez un nombre entre 1 et 100 :",
            "Ce n'est pas un nombre valide",
            tonumber,
            function(nb) return nb ~= nil end
        )
		nombre_d_essais = nombre_d_essais + 1

		if nombre_a_deviner < supposition_utilisateur then
			print("Le nombre est plus bas !")
		end

		if nombre_a_deviner > supposition_utilisateur then
			print("Le nombre est plus haut !")
		end
	until supposition_utilisateur == nombre_a_deviner

	print("Vous avez gagné en " .. nombre_d_essais .. " essais !")

    reponse_utilisateur = lire_validee(
        "Voulez-vous continuer ? oui / non",
        "Ce n'est pas une réponse valide.",
        string.lower,
        function(reponse) return reponse == "oui" or reponse == "non" end
    )
until reponse_utilisateur == "non"

print("Au revoir !")
```

</div>
