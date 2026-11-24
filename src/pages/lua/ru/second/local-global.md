Пока что мы объявляли переменные Lua с таким синтаксисом:

```lua
my_variable = "value"
```

И это работает, очевидно, как мы видели. Но это не всегда может быть лучшим способом сделать это.
Возьмём такой сценарий:

<div class="runner">

```lua
function my_function(a)
    b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```
</div>

`b` всё ещё определена _вне_ функции. Что иногда может быть желаемым. Но в большинстве случаев это не ожидаемое поведение. Чаще всего вы хотите создать переменную, которая существует только внутри функции, внутри цикла, в пределах отдельной области видимости.

Чтобы определить значение, присутствующее только в особой области видимости, мы можем просто поставить ключевое слово `local` перед именем переменной, вот так:

```lua
local my_variable = "value"
```

С этим переменная будет существовать, пока не достигнет либо конца блока (`end`, `repeat ... until`, ...), в котором она была определена, либо конца файла, если она была определена вне блока. С этим мы можем исправить наш пример:

<div class="runner">

```lua
function my_function(a)
    local b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```

</div>

<div class="note">

Значения `for` по умолчанию являются `local`

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

Объявление переменной и присваивание ей значения — две разные вещи:

- **Объявление** создаёт переменную с областью видимости из `local`: `local my_variable`
- **Присваивание** даёт переменной значение: `my_variable = "value"`

Запись `local my_variable = "value"` просто делает и то, и другое сразу.

Вы также можете объявить переменную, не давая ей значения. Она будет содержать `nil`, пока вы что-нибудь ей не присвоите:

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

Пишите `local` только один раз на переменную: второй `local user_guess = 42` создаст новую переменную (возможно, в новой области видимости) вместо изменения первой.

</div>

И мы можем закончить нашу игру этим финальным примером:

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    local user_input

    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        local is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

local scores = {}

repeat 
	local number_of_tries = 0
	local number_to_guess = math.random(1, 100)

	print("Попробуйте угадать случайное число от 1 до 100!")

	repeat
        local user_guess = validated_read(
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

    local user_answer = validated_read(
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
