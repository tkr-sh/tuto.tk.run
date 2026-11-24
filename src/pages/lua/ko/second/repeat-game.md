이제 한 번의 실행에서 여러 게임 세션을 가질 수 있고, 사용자가 언제 멈출지 결정할 수 있다면 어떨까요?

게임을 반복 가능하게 만들려면 다음과 같이 프로그램을 또 다른 `repeat ... until` 루프로 감싸면 됩니다.

```lua
repeat
	...
	print("당신은 ...번 만에 맞혔습니다!")

	repeat 
		print("계속하시겠습니까? yes 또는 no")
		user_answer = io.read()
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("안녕히 가세요!")
```

여기서는 특별한 키워드 `or`를 사용합니다. 이것은 두 조건을 결합하며, 적어도 하나가 참이면 `true`입니다.

예를 들어 `true` 또는 `false`를 가질 수 있는 부울 `a`와, 역시 `true` 또는 `false`를 가질 수 있는 또 다른 부울 `b`가 있다고 합시다. 그러면 표현식 `a or b`는 다음과 같습니다.

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b`는 `a`와 `b`가 모두 `false`일 때만 `false`입니다.
이것은 `and` 연산자의 반대입니다. `and`는 결과가 `true`가 되려면 `a`와 `b`가 모두 `true`여야 합니다.

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## 대소문자

Lua에서 `"A"`, `"B"`, `"C"`처럼 대문자가 있는 문자열은 `"a"`, `"b"`, `"c"`와 같지 않습니다. 즉, 사용자가 `"Yes"`를 입력하면 작동하지 않고, 사용자가 `"Yes"`를 입력했음에도 `repeat` 루프를 다시 하게 됩니다. 이것은 아마 우리가 원하는 것이 아닐 것입니다……

다행히 Lua에는 문자열을 모두 대문자나 모두 소문자로 변환할 수 있는 함수가 있습니다. `string.upper`와 `string.lower`입니다.

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

이를 통해 다음을 할 수 있습니다.

<div class="runner">

```lua
repeat 
	print("계속하시겠습니까? yes / no")
	user_answer = string.lower(io.read())
	if user_answer ~= "yes" and user_answer ~= "no" then
		print("유효한 답변이 아닙니다.")
	end
until user_answer == "yes" or user_answer == "no"
```

</div>

이렇게 하면 `string.lower()` 덕분에 `user_answer`는 항상 소문자가 됩니다.
또한 응답이 `"yes"`도 `"no"`도 아닐 때 메시지를 출력하는 검사도 추가했습니다.
그러면 다음과 같은 완전한 프로그램이 됩니다.

<div class="runner">

```lua
repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")

	repeat
		repeat
			user_guess = tonumber(io.read())
			if user_guess == nil then 
				print("유효한 숫자가 아닙니다!")
			end
		until user_guess ~= nil
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("숫자가 더 작습니다!")
		end

		if number_to_guess > user_guess then
			print("숫자가 더 큽니다!")
		end
	until user_guess == number_to_guess

	print("당신은 " .. number_of_tries .. "번 만에 맞혔습니다!")

	repeat 
		print("계속하시겠습니까? yes / no")
		user_answer = string.lower(io.read())
		if user_answer ~= "yes" and user_answer ~= "no" then
			print("유효한 답변이 아닙니다.")
		end
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("안녕히 가세요!")
```

</div>
