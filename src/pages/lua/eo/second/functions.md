Vi eble rimarkis, ke ni faras similan kodblokon dufoje por la validigo de enigo:

```lua
repeat
    print("Provu diveni nombron:")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("Ĉi tio ne estas valida nombro!")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("Ĉu vi volas daŭrigi? jes / ne")
    user_answer = string.lower(io.read())
    if user_answer ~= "jes" and user_answer ~= "ne" then
        print("Ĉi tio ne estas valida respondo.")
    end
until user_answer == "jes" or user_answer == "ne"
```

Estas klara komuna duplikata ŝablono, kiun mi provos ilustri per pseŭdo/malvalida sintakso limigita per `<` kaj `>`.

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## Bazoj de funkcio

Kiam vi skribas programon, ofte ekzistas regulo sekvenda, nome skribi aferojn laŭmodula maniero, por ke ĝi estu utila estonte kaj sufiĉe ĝenerala por trakti la plej multajn kazojn, kaj ĝuste tion helpas nin atingi funkcioj.

Funkcio esence estas aro da instrukcioj, kiujn programo plenumos. Funkcio povas preni parametrojn kaj povas redoni valoron. Ni prenu ĉi tiun funkcion ekzemple:

```lua
function double(my_number)
    return my_number * 2
end
```

La nomo de ĉi tiu funkcio estas `double` kaj ĝi prenas unu solan parametron nomatan `my_number`. La rezulto de ĉi tiu funkcio, tio, kion ĝi redonas, estas unu sola valoro: `my_number * 2`. Vi povas voki funkcion specifante la nomon kaj poste, inter parentezoj, apartigitajn per `,`, la argumentojn tiel `my_function(value1, value2)`. En nia kazo:

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## Reen al nia ludo!

Do ni optimumigu nian enigotraktadon! La unua afero, kiun ni pripensu, estas: de kio dependas nia funkcio? Tio necesas por scii, kiujn argumentojn ĝi bezonas. En nia kazo:
* Invita mesaĝo, la mesaĝo diranta al la uzanto enigi valoron
* Erara mesaĝo, la mesaĝo montrata, kiam la enigo ne estas valida
* Transformo, transformo, kiun ni aplikas al la ĉeno, kiun ni legas de la uzanto (`tonumber`, `string.lower`, `string.upper`, ...)
* Kondiĉo, kontrolo por scii, ĉu la enigo kongruas kun tio, kion ni atendas post aplikado de la transformo.

La `prompt_message` kaj `error_message` estas iom facilaj, ili estas simple ĉenoj.<br/>

`transform` kaj `condition` estas iom pli komplikaj.<br>
`transform` prenas la enigon de la uzanto kaj resendas modifitan version de ĝi. Ĉi tio klare estas kazo de funkcio: io, kio prenas ion kiel enigon kaj resendas ion.<br/>
`condition` ankaŭ tre similas: ĝi prenas kiel argumenton la rezulton de `transform` kaj redonas bulean valoron (`true`/`false`) depende de la rezulto. Ĉi tio denove klare estas kazo, kie necesas `function`.

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end
```

Rimarku la `not`! La ŝlosilvorto `not` inversigas bulean valoron: `true` => `false`, `false` => `true`.

En nia kazo ni povas simple skribi ĉi tion por la enigotraktado por akiri la nombron de la uzanto:

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "Provu diveni nombron:",
    "Ĉi tio ne estas valida nombro",
    tonumber,
    is_not_nil
)
```

Ĉi tio produktos la saman aferon kiel:

```lua
repeat
    print("Provu diveni nombron:")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("Ĉi tio ne estas valida nombro!")
    end
until is_valid
```

## Enlinigo de funkcioj

Rimarku, ke ni difinis `is_not_nil` nur por uzi ĝin unufoje. Anstataŭ deklari funkcion kun nomo, kiam ni ne bezonas ĝin, Lua permesas al ni simple forigi la nomon, kiam ni ne volas uzi ĝin aliloke, tiel:

```lua
local double = function(number)
    return number * 2
end
```

En nia kazo, ni povas uzi ĝin por fari:

```lua
validated_read(
    "Provu diveni nombron:",
    "Ĉi tio ne estas valida nombro",
    tonumber,
    function(nb) return nb ~= nil end
)
```

Kio donas al ni nian finan programon:

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("Provu diveni la hazardan nombron inter 1 kaj 100!")

	repeat
        user_guess = validated_read(
            "Enigu nombron inter 1 kaj 100:",
            "Ĉi tio ne estas valida nombro",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("La nombro estas pli malgranda!")
		end

		if number_to_guess > user_guess then
			print("La nombro estas pli granda!")
		end
	until user_guess == number_to_guess

	print("Vi venkis en " .. number_of_tries .. " provoj!")

    user_answer = validated_read(
        "Ĉu vi volas daŭrigi? jes / ne",
        "Ĉi tio ne estas valida respondo.",
        string.lower,
        function(response) return response == "jes" or response == "ne" end
    )
until user_answer == "ne"

print("Ĝis revido!")
```

</div>
