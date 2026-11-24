Первое, что вы, возможно, заметили, — это то, что наша игра сейчас *падает* (crash), когда мы вводим число, которое не является допустимым числом:

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("Загаданное число меньше!")
end
```

</div>

что, если вы не введёте число, даст вам:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

Но что это значит?

Ключевая концепция здесь — `nil`. При знакомстве с типами Lua мы кратко видели тип `nil`:

> `nil`: Пусто. Цель `nil` — сказать, что у вас нет значения.

<div style="height: 3px"></div>

И это _именно_ то, что делает `tonumber`:

- Если `io.read()` — это строка, представляющая число, например `"3.14"` или `"42"`, Lua знает, как преобразовать её из типа `string` в тип `number`.
- Иначе, если `io.read()` — это строка, которая не похожа на `number`, `tonumber` просто говорит: «Я не знаю, как преобразовать это в число, давай просто вернём `nil`».

Поэтому, если в `io.read()` мы не передадим корректное представление числа, у нас просто будет `nil`:

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## Хорошо, но почему тогда возникает ошибка?

Хороший вопрос! Пока что `user_guess` — это просто `nil`. Ошибка возникает, когда мы пытаемся сравнить с помощью оператора `<` число `number` с `nil` в этой строке:

```lua
if 10 < user_guess then
```

И ошибка хорошо описана в сообщении об ошибке:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` означает строку, в которой происходит ошибка, в данном случае нашу строку сравнения.
- `attempt to compare number with nil` — это причина, по которой произошла ошибка.

И это совершенно логично! Что значит сравнить, в смысле больше или меньше, ничто с числом? Это `false`? Это `true`? Нет, сам вопрос просто не имеет смысла. Поэтому Lua просто выдаёт ошибку.

## Обработка ошибки
Самый быстрый способ справиться с этим — просто проверить, является ли `user_guess` равным `nil`, и это можно сделать с помощью оператора `==`:

```lua
if user_guess == nil then
	print("Это не допустимое число")
end
```

Но в этом случае мы не хотим просто вывести что-то и продолжить цикл, потому что это засчитается как попытка из-за:

```lua
number_of_tries = number_of_tries + 1
```

Простой способ сделать это — спрашивать `user_guess`, пока он не станет не `nil`, с помощью оператора `~=`, то есть пока он не станет допустимым `number`:

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

Мы также можем добавить сообщение об ошибке на случай, если это так, с помощью:

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("Это не допустимое число!")
	end
until user_guess ~= nil
```

Что даёт нам эту полную программу:


<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("Попробуйте угадать случайное число от 1 до 100!")

repeat
	repeat
		user_guess = tonumber(io.read())
		if user_guess == nil then 
			print("Это не допустимое число!")
		end
	until user_guess ~= nil
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("Загаданное число меньше!")
	end

	if number_to_guess > user_guess then
		print("Загаданное число больше!")
	end
until user_guess == number_to_guess

print("Вы угадали за " .. number_of_tries .. " попыток!")
```

</div>
