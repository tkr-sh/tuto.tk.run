이번에는 처음으로 단일 줄에 집중하지 않겠습니다! 현재 줄은 그냥

```lua
repeat
```

이지만, 그 자체로는 아무것도 하지 않습니다. `until`과 함께 사용합니다.

```lua
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
```

그런데 우선…… `repeat`와 `until`은 무엇이고 왜 이렇게 특별한가요? 그냥 이상한 변수 아닌가요?

변수의 정의를 기억할지도 모르겠습니다.

> `the_name_of_the_variable`은 무엇이든 될 수 있습니다. `true`, `false`, `nil`(그리고 나중에 보게 될 다른 것들)은 제외입니다.

그리고 "나중"…… 이 바로 지금입니다!

## 키워드

`repeat`와 `until`은 우리가 "키워드"라고 부르는 것입니다. 키워드는 특별한 이름으로, 변수처럼 값을 가지거나 함수처럼 계산을 하는 대신, Lua에게 특정 결정을 내리라고 지시하는 명령입니다.

우리의 `repeat`/`until` 예시에서는 `repeat`와 `until` 사이에 있는 모든 것이 특정 조건이 충족될 때까지 반복됩니다. 실행이 `until`에 도달하면 다시 실행할지 확인하고, 필요하면 코드가 `repeat`로 돌아가 반복합니다.

이런 동작, 즉 _무언가_ 가 일어날 때까지 코드를 반복하는 것은 "루프"라고 불리며 프로그래밍에서 아주 흔한 개념입니다.

대부분의 경우, 당신은 어느 시점에서 루프를 빠져나가고 싶을 것입니다……

### 그런데 언제 빠져나가야 하는지는 어떻게 알죠?

`until` 같은 루프를 사용할 때는 언제 루프를 멈출지 알려 주기 위해 `boolean`(`true` 또는 `false`)을 전달해야 합니다.

그리고 대부분의 경우 관계 연산자를 사용해 그것을 합니다.

- `==`: 두 값이 같은지 확인합니다. **1개의 `=`와 혼동하지 마세요**. 1개의 `=`는 값을 할당하는 것이고, 2개의 `=`(`==`)는 두 값이 같은지 확인하는 것입니다. 같으면 값은 `true`, 아니면 `false`입니다.
- `~=`: 두 값이 다른지 확인합니다. 다르면 값은 `true`, 아니면 `false`입니다.
- `>`: 왼쪽 값이 오른쪽 값보다 큰지 확인합니다. 그렇다면 값은 `true`, 아니면 `false`입니다.
- `<`: 왼쪽 값이 오른쪽 값보다 작은지 확인합니다. 그렇다면 값은 `true`, 아니면 `false`입니다.
- `>=`: 왼쪽 값이 오른쪽 값보다 크거나 **같은지** 확인합니다. 그렇다면 값은 `true`, 아니면 `false`입니다.
- `<=`: 왼쪽 값이 오른쪽 값보다 작거나 **같은지** 확인합니다. 그렇다면 값은 `true`, 아니면 `false`입니다.


<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```

</div>

### 루프의 종류

- `repeat`/`until`: 특정 조건이 충족될 때까지 코드를 반복하고, 조건이 충족되면 멈춥니다.

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while`: while 루프는 `repeat`/`until` 루프와 매우 비슷하지만 반대입니다! 조건이 충족되는 동안 루프합니다! 조건 뒤에 `do` 키워드를 사용해 코드 실행을 시작하고, `end` 키워드를 보면 시작 지점으로 돌아가 루프합니다. `repeat`를 사용할 수 있는 대부분의 상황에서 `while`도 사용할 수 있습니다. 주로 개인 취향입니다.

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for`: 마지막 루프는 `for` 루프이고 조금 다릅니다. 얼마나 반복할지 이미 알고 있기 때문입니다. 어떤 숫자에서 어떤 숫자까지 반복할지 정의합니다. 한 번의 반복이 끝나면 자동으로 `age`를 `1` 증가시킵니다!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>


## 우리의 경우로 돌아가서

우리의 경우, 올바른 숫자를 찾을 때까지 사용자에게 계속 숫자를 물어보고 싶습니다. 따라서 우리는
```lua
repeat
    ...
until user_guess == number_to_guess
```
