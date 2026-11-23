Now that we know all of that, we can make our history of game work!

To make a history of our scores, we can create a new table `scores` at the very start of our program, and every time we finish a game, we add the `number_of_tries` to it as `scores[#scores + 1]`, meaning, the last element.

Once the game is over, we can just iterate through the table with `ipairs` and print all of our scores!


This gives us our final program:

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

	print("Try to guess the random number between 1 and 100!")

	repeat
        user_guess = validated_read(
            "Enter a number between 1 and 100:",
            "This is not a valid number",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("The number is lower!")
		end

		if number_to_guess > user_guess then
			print("The number is higher!")
		end
	until user_guess == number_to_guess

	print("You won in " .. number_of_tries .. " tries!")

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
        "Do you want to continue? yes / no",
        "This is not a valid answer.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("Scores:")

for game_number, score in ipairs(scores) do
    print("Game " .. game_number .. ": finished in " .. score .. " tries!")
end

print("Goodbye!")
```

</div>
