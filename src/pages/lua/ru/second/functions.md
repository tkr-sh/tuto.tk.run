Вы, возможно, заметили, что мы дважды делаем похожий блок кода для проверки ввода:

```lua
repeat
    print("Попробуйте угадать число:")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("Это не допустимое число!")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("Хотите продолжить? да / нет")
    user_answer = string.lower(io.read())
    if user_answer ~= "да" and user_answer ~= "нет" then
        print("Это не допустимый ответ.")
    end
until user_answer == "да" or user_answer == "нет"
```

Есть явный общий дублирующийся шаблон, который я попробую проиллюстрировать с помощью псевдо/недопустимого синтаксиса, ограниченного символами `<` и `>`.

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## Основы функции

Когда вы пишете программу, часто есть правило, которого нужно придерживаться: писать вещи модульно, чтобы это могло пригодиться в будущем и было достаточно универсальным, чтобы обрабатывать большинство случаев, и именно этого помогают достичь функции.

Функция — это, по сути, набор инструкций, которые выполнит программа. Функция может принимать параметры и может возвращать значение. Возьмём, например, эту функцию:

```lua
function double(my_number)
    return my_number * 2
end
```

Имя этой функции — `double`, и она принимает один параметр под названием `my_number`. Результат этой функции, то, что она возвращает, — это одно значение: `my_number * 2`. Вы можете вызвать функцию, указав имя, а затем в круглых скобках, разделённые с помощью `,`, аргументы, вот так `my_function(value1, value2)`. В нашем случае:

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## Возвращаемся к нашей игре!

Итак, давайте оптимизируем обработку ввода! Первое, на что следует обратить внимание: от чего зависит наша функция? Это необходимо знать, чтобы понять, какие аргументы ей нужны. В нашем случае:
* Сообщение-приглашение, сообщение, которое говорит пользователю ввести значение
* Сообщение об ошибке, сообщение, которое показывается, когда ввод недопустим
* Преобразование, преобразование, которое мы применяем к строке, прочитанной от пользователя (`tonumber`, `string.lower`, `string.upper`, ...)
* Условие, проверка, чтобы узнать, соответствует ли ввод тому, что мы ожидаем, после применения преобразования.

`prompt_message` и `error_message` довольно просты, это просто строки.<br/>

`transform` и `condition` немного сложнее.<br>
`transform` берёт ввод пользователя и возвращает его изменённую версию. Это явно случай функции: что-то, что получает что-то на вход и возвращает что-то обратно.<br/>
`condition` тоже очень похожа: она принимает в качестве аргумента результат `transform` и возвращает `boolean` (`true`/`false`) в зависимости от результата. Это опять же явно случай, когда нужна `function`.

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

Обратите внимание на `not`! Ключевое слово `not` переворачивает `boolean`: `true` => `false`, `false` => `true`.

В нашем случае мы можем просто написать это для обработки ввода при получении числа от пользователя:

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "Попробуйте угадать число:",
    "Это не допустимое число",
    tonumber,
    is_not_nil
)
```

Это даст тот же результат, что и:

```lua
repeat
    print("Попробуйте угадать число:")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("Это не допустимое число!")
    end
until is_valid
```

## Встраивание функций

Обратите внимание, что мы определили `is_not_nil` только для того, чтобы использовать её один раз. Вместо того чтобы объявлять функцию с именем, когда она нам не нужна, Lua позволяет нам просто убрать имя, когда мы не хотим использовать её где-то ещё, вот так:

```lua
local double = function(number)
    return number * 2
end
```

В нашем случае мы можем использовать это, чтобы сделать:

```lua
validated_read(
    "Попробуйте угадать число:",
    "Это не допустимое число",
    tonumber,
    function(nb) return nb ~= nil end
)
```

Что даёт нам нашу финальную программу:

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

    user_answer = validated_read(
        "Хотите продолжить? да / нет",
        "Это не допустимый ответ.",
        string.lower,
        function(response) return response == "да" or response == "нет" end
    )
until user_answer == "нет"

print("До свидания!")
```

</div>
