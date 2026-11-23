Maintenant que nous savons tout cela, nous pouvons faire fonctionner notre historique de parties !

Pour créer un historique de nos scores, nous pouvons créer une nouvelle table `scores` au tout début de notre programme, et chaque fois que nous terminons une partie, nous y ajoutons le `nombre_d_essais` sous la forme `scores[#scores + 1]`, c'est-à-dire le dernier élément.

Une fois la partie terminée, nous pouvons simplement parcourir la table avec `ipairs` et afficher tous nos scores !


Cela nous donne notre programme final :

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

scores = {}

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

    scores[#scores + 1] = nombre_d_essais

    reponse_utilisateur = lire_validee(
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
