Теперь, когда мы всё это знаем, мы можем заставить нашу историю игр работать!

Чтобы создать историю наших результатов, мы можем создать новую таблицу `scores` в самом начале нашей программы, и каждый раз, когда мы заканчиваем игру, добавлять в неё `number_of_tries` как `scores[#scores + 1]`, то есть как последний элемент.

Как только игра окончена, мы можем просто перебрать таблицу с помощью `ipairs` и вывести все наши результаты!


Это даёт нам нашу финальную программу:

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

	print("Попробуйте угадать случайное число от 1 до 100!")

	repeat
        user_guess = validated_read(
            "Введите число от 1 до 100:",
            "Это не допустимое число",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("Загаданное число меньше!")
		end

		if number_to_guess > user_guess then
			print("Загаданное число больше!")
		end
	until user_guess == number_to_guess

	print("Вы угадали за " .. number_of_tries .. " попыток!")

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
        "Хотите продолжить? да / нет",
        "Это не допустимый ответ.",
        string.lower,
        function(response) return response == "да" or response == "нет" end
    )
until user_answer == "нет"

print("Результаты:")

for game_number, score in ipairs(scores) do
    print("Игра " .. game_number .. ": завершена за " .. score .. " попыток!")
end

print("До свидания!")
```

</div>
