Это первая программа, которую мы изучим! Не волнуйтесь, мы разберём её ОЧЕНЬ медленно, так что всё в порядке, если вы пока ничего не понимаете!

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("Попробуйте угадать случайное число от 1 до 100!")

repeat
	user_guess = tonumber(read())
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

Что делает эта программа — создаёт игру «угадай число»:

```
Попробуйте угадать случайное число от 1 до 100!
50
Загаданное число больше!
75
Загаданное число меньше!
62
Вы угадали за 3 попыток!
```
