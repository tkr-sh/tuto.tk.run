А что, если бы у нас могло быть несколько игровых сессий за одно выполнение программы, и пользователь сам решал бы, когда остановиться?

Чтобы сделать игру повторяемой, мы просто оборачиваем программу в ещё один цикл `repeat ... until`, вот так:

```lua
repeat
	...
	print("Вы угадали за ... попыток!")

	repeat 
		print("Хотите продолжить? да ИЛИ нет")
		user_answer = io.read()
	until user_answer == "да" or user_answer == "нет"
until user_answer == "нет"

print("До свидания!")
```

Мы используем специальное ключевое слово `or`, которое объединяет два условия и равно `true`, если хотя бы одно из них истинно.

Скажем, например, у нас есть `boolean` `a`, который может быть `true` или `false`, и ещё один `boolean` `b`, который также может быть `true` или `false`. Тогда выражение `a or b` будет:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` равно `false`, только когда `a` и `b` равны `false`.
Это противоположность оператору `and`, который требует, чтобы `a` и `b` были `true`, чтобы он был `true`

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## Регистр

В Lua строка с заглавными буквами, например `"A"`, `"B"`, `"C"`, ... не то же самое, что `"a"`, `"b"`, `"c"`, .... Это означает, что если пользователь введёт `"Yes"`, это не сработает, и пользователь снова пройдёт цикл `repeat`, хотя ввёл `"Yes"`. Вероятно, это не то, что мы хотим...

К счастью, в Lua есть функции, которые позволяют преобразовать строку во все заглавные или все строчные буквы: `string.upper` и `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

Что позволяет нам сделать следующее

<div class="runner">

```lua
repeat 
	print("Хотите продолжить? да / нет")
	user_answer = string.lower(io.read())
	if user_answer ~= "да" and user_answer ~= "нет" then
		print("Это не допустимый ответ.")
	end
until user_answer == "да" or user_answer == "нет"
```

</div>

С этим `user_answer` всегда будет в нижнем регистре благодаря `string.lower()`.
Мы также добавляем проверку, чтобы вывести сообщение, если ответ не является ни `"да"`, ни `"нет"`. 
Что даёт нам эту полную программу:

<div class="runner">

```lua
repeat 
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

	repeat 
		print("Хотите продолжить? да / нет")
		user_answer = string.lower(io.read())
		if user_answer ~= "да" and user_answer ~= "нет" then
			print("Это не допустимый ответ.")
		end
	until user_answer == "да" or user_answer == "нет"
until user_answer == "нет"

print("До свидания!")
```

</div>
