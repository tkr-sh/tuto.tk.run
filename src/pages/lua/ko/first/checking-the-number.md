이제 우리는 다음을 할 수 있다는 것을 압니다.

1. 무작위 숫자 생성하기
2. 사용자 입력 받기
3. 반복하기

이제 사용자가 추측한 숫자가 비밀 숫자보다 작은지 큰지 알려 주어 사용자를 도와야 합니다.

그를 위해 앞으로 자주 사용하게 될 새로운 키워드 `if`를 사용합니다.

`if`가 하는 일은 기본적으로 조건에 따라 코드를 실행하는 것입니다. 루프는 반복하기 위해 조건을 확인하고, `if`는 한 번만 실행하기 위해 조건을 확인합니다.

예를 들어, 사용자가 `42`를 입력했는지 확인하고, 그렇다면 특별한 메시지로 인사할 수 있습니다.
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("좋은 선택입니다!")
end
```

</div>

그래서 우리 게임에서는

```lua
if number_to_guess < user_guess then
    print("숫자가 더 작습니다!")
end

if number_to_guess > user_guess then
    print("숫자가 더 큽니다!")
end
```

숫자가 더 작은지 큰지 확인하고, 그에 따라 사용자를 돕는 메시지를 출력합니다.

사용자가 정확한 숫자를 찾으면 다음 때문에 루프가 멈춥니다.

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("숫자가 더 작습니다!")
end

if number_to_guess > user_guess then
    print("숫자가 더 큽니다!")
end

if number_to_guess == user_guess then
    print("정답입니다!")
end
```

</div>

그리고 이것이 끝나면, 이제 좋은 결말만 만들면 됩니다……
