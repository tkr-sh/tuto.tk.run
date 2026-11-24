이것은 우리가 공부할 첫 번째 프로그램입니다! 걱정하지 마세요. 아주 아주 천천히 진행할 것이므로, 아직 아무것도 이해하지 못해도 괜찮습니다!

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")

repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("숫자가 더 작습니다!")
	end

	if number_to_guess > user_guess then
		print("숫자가 더 큽니다!")
	end
until user_guess == number_to_guess

print("당신은 " .. number_of_tries .. "번 만에 맞혔습니다!")
```

이 프로그램이 하는 일은 "숫자 찾기" 게임을 만드는 것입니다.

```
1에서 100 사이의 무작위 숫자를 맞혀 보세요!
50
숫자가 더 큽니다!
75
숫자가 더 작습니다!
62
당신은 3번 만에 맞혔습니다!
```
