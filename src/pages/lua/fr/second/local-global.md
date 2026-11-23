Jusqu'à présent, nous n'avons déclaré des variables Lua qu'avec cette syntaxe :

```lua
ma_variable = "valeur"
```

Et cela fonctionne, évidemment, comme nous l'avons vu. Mais ce n'est pas toujours la meilleure façon de faire.
Prenons ce scénario :

<div class="runner">

```lua
function ma_fonction(a)
    b = a * 2
    return a + b
end

print(ma_fonction(2))
print(b)
```
</div>

`b` est toujours défini _en dehors_ de la fonction. Ce qui, parfois, peut être voulu. Mais la plupart du temps, ce n'est pas le comportement attendu. La plupart du temps, vous voulez créer une variable qui n'existe que dans une fonction, dans une boucle, dans une portée unique.

Pour définir une valeur qui n'existe que dans une portée particulière, nous pouvons simplement placer le mot-clé `local` avant le nom de la variable, comme ceci :

```lua
local ma_variable = "valeur"
```

Avec cela, la variable existera jusqu'à ce qu'elle atteigne soit la fin du bloc (`end`, `repeat ... until`, ...) dans lequel elle a été définie, soit la fin du fichier si elle a été définie en dehors d'un bloc. Avec cela, nous pouvons corriger notre exemple :

<div class="runner">

```lua
function ma_fonction(a)
    local b = a * 2
    return a + b
end

print(ma_fonction(2))
print(b)
```

</div>

<div class="note">

Les valeurs de `for` sont `local` par défaut

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

Déclarer une variable et lui assigner une valeur sont deux choses différentes :

- **Déclarer** crée la variable avec la portée de `local` : `local ma_variable`
- **Assigner** donne une valeur à une variable : `ma_variable = "valeur"`

Écrire `local ma_variable = "valeur"` fait simplement les deux à la fois.

Vous pouvez aussi déclarer une variable sans lui donner de valeur. Elle contiendra `nil` jusqu'à ce que vous lui assigniez quelque chose :

```lua
local supposition_utilisateur
print(supposition_utilisateur) -- nil

supposition_utilisateur = 42
print(supposition_utilisateur) -- 42
```

N'écrivez `local` qu'une seule fois par variable : un second `local supposition_utilisateur = 42` créerait une nouvelle variable (éventuellement dans une nouvelle portée) au lieu de modifier la première.

</div>

Et nous pouvons terminer notre jeu avec cet exemple final :

<div class="runner">

```lua
function lire_validee(message_invite, message_erreur, transformer, condition)
    local saisie_utilisateur

    repeat 
        print(message_invite)
        saisie_utilisateur = transformer(io.read())
        local est_valide = condition(saisie_utilisateur)

        if not est_valide then
            print(message_erreur)
        end
    until est_valide

    return saisie_utilisateur
end

local scores = {}

repeat 
	local nombre_d_essais = 0
	local nombre_a_deviner = math.random(1, 100)

	print("Essayez de deviner le nombre aléatoire entre 1 et 100 !")

	repeat
        local supposition_utilisateur = lire_validee(
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

    scores[#scores + 1] = nombre_d_essais

    local reponse_utilisateur = lire_validee(
        "Voulez-vous continuer ? oui / non",
        "Ce n'est pas une réponse valide.",
        string.lower,
        function(reponse) return reponse == "oui" or reponse == "non" end
    )
until reponse_utilisateur == "non"

print("Scores :")

for numero_de_partie, score in ipairs(scores) do
    print("Partie " .. numero_de_partie .. " : terminée en " .. score .. " essais !")
end

print("Au revoir !")
```

</div>
