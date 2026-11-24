이제 남은 것은 시도 횟수를 출력하는 것뿐입니다.

이전에 우리는 사용자 데이터를 받은 직후

```lua
number_of_tries = number_of_tries + 1
```

로 그것을 증가시켰습니다.

하지만 이제는 축하 메시지와 동시에 몇 번 만에 맞혔는지 출력하고 싶습니다.

첫 번째 게임이 끝나기 전에 마지막 연산자를 배울 시간입니다.

`..`: 2개의 값을 연결합니다.

<div class="runner">

```lua
print("벌써 " .. 4 .. "년이 지났습니다")
```

</div>

이것을 염두에 두면 멋진 메시지를 만들 수 있습니다.

```lua
print("당신은 " .. number_of_tries .. "번 만에 맞혔습니다!")
```

그리고 마침내 우리의 게임입니다.

---

<div class="runner">

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

</div>

---

## 와.

와! 대단하지 않나요?

이제 당신은 프로그래머가 될 준비가 되었습니다 :)!

## 끝난 건가요?

그렇기도 하고 아니기도 합니다.

안타깝게도, 저는 한 가지를 지나치게 단순화했습니다. 보통 `read`와 `random`을 그렇게 바로 실행할 수는 없습니다. Lua는 내장 함수를 몇 가지 범주로 나누는 것을 좋아합니다. `math`(수학 함수용), `io`(입출력 상호작용용), `os`(운영체제와 상호작용하기 위해)……

따라서 범주에 속한 특정 함수를 사용하고 싶을 때는 `category.the_function()`처럼 호출해야 합니다.

우리의 경우 `random`은 `math` 범주에 속하고, `read`는 `io` 범주에 속합니다. `print`와 `tonumber`는 전역이므로 아무것도 지정할 필요가 없습니다.

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")

repeat
	user_guess = tonumber(io.read())
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

## 축하합니다, 이제 첫 번째 Lua 프로그램을 완성했습니다! 🎉

이제 기초를 갖추었으니, 현재 게임을 개선해 새로운 기능을 추가하고, 그 과정에서 새로운 것들을 배워 보겠습니다!
