가장 먼저 눈치챘을 수 있는 것은, 현재 우리 게임이 유효한 숫자가 아닌 값을 입력하면 *충돌* 한다는 것입니다.

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("숫자가 더 작습니다!")
end
```

</div>

숫자를 입력하지 않으면 다음과 같이 나옵니다.

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

그런데 이것은 무슨 뜻일까요?

여기서 핵심 개념은 `nil`입니다. Lua 타입을 소개할 때 `nil` 타입을 잠깐 보았습니다.

> `nil`: 비어 있음. `nil`의 목적은 값이 없다는 것을 말하는 것입니다.

<div style="height: 3px"></div>

그리고 이것이 _바로_ `tonumber`가 하는 일입니다.

- `io.read()`가 `"3.14"`나 `"42"`처럼 숫자를 나타내는 문자열이면, Lua는 그것을 `string` 타입에서 `number` 타입으로 변환하는 방법을 압니다.
- 그렇지 않고 `io.read()`가 `number`처럼 보이지 않는 문자열이면, `tonumber`는 "이것을 숫자로 변환하는 방법을 모르겠으니 그냥 `nil`을 반환하자"라고 말합니다.

따라서 `io.read()`에서 올바른 숫자 표현을 얻지 못하면 그냥 `nil`이 됩니다.

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## 좋아요, 그런데 왜 오류가 발생하나요?

좋은 질문입니다! 지금 `user_guess`는 그냥 `nil`입니다. 오류는 이 줄에서 `<` 연산자로 `number`와 `nil`을 비교하려고 할 때 발생합니다.

```lua
if 10 < user_guess then
```

그리고 오류 메시지에 잘 설명되어 있습니다.

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3`은 오류가 발생한 줄을 의미하며, 이 경우 우리의 비교 줄입니다.
- `attempt to compare number with nil`은 오류가 발생한 이유입니다.

이것은 정말 말이 됩니다! 크거나 작다는 관점에서 "아무것도 없는 것"을 숫자와 비교한다는 것은 무슨 의미일까요? `false`일까요? `true`일까요? 아니요, 그 질문 자체가 말이 되지 않습니다. 그래서 Lua는 그냥 오류를 냅니다.

## 오류 처리하기
이를 처리하는 가장 빠른 방법은 `user_guess`가 `nil`인지 확인하는 것이며, 이는 `==` 연산자로 할 수 있습니다.

```lua
if user_guess == nil then
	print("유효한 숫자가 아닙니다")
end
```

하지만 이 경우에는 무언가를 출력하고 루프를 계속하기만 해서는 안 됩니다. 다음 줄 때문에 그것이 한 번의 추측으로 계산되기 때문입니다.

```lua
number_of_tries = number_of_tries + 1
```

간단한 방법은 `~=` 연산자를 사용해 `user_guess`가 `nil`이 아닐 때까지, 즉 유효한 `number`가 될 때까지 계속 묻는 것입니다.

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

`nil`인 경우를 위해 오류 메시지를 추가할 수도 있습니다.

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("유효한 숫자가 아닙니다!")
	end
until user_guess ~= nil
```

그러면 다음과 같은 완전한 프로그램이 됩니다.


<div class="runner">

```lua
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
```

</div>
