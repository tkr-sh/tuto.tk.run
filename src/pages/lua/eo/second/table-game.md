Nun, kiam ni scias ĉion tion, ni povas igi nian ludhistorion funkcii!

Por krei historion de niaj poentoj, ni povas krei novan tabelon `scores` ĉe la tre komenco de nia programo, kaj ĉiufoje kiam ni finas ludon, ni aldonas la `number_of_tries` al ĝi kiel `scores[#scores + 1]`, tio signifas la lastan elementon.

Post kiam la ludo finiĝis, ni povas simple trairi la tabelon per `ipairs` kaj presi ĉiujn niajn poentojn!


Ĉi tio donas al ni nian finan programon:

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

scores = {}

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

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
        "Ĉu vi volas daŭrigi? jes / ne",
        "Ĉi tio ne estas valida respondo.",
        string.lower,
        function(response) return response == "jes" or response == "ne" end
    )
until user_answer == "ne"

print("Poentoj:")

for game_number, score in ipairs(scores) do
    print("Ludo " .. game_number .. ": finita en " .. score .. " provoj!")
end

print("Ĝis revido!")
```

</div>
