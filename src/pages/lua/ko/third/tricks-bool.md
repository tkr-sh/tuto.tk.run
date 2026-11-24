## 무엇이 `true`로 간주되나요?

조건에서 Lua는 `true`와 `false`만 허용하지 않습니다. 어떤 값이든 사용할 수 있습니다. 규칙은 간단합니다. `false`와 `nil`은 거짓으로 간주되고, **그 외의 모든 것**은 참으로 간주됩니다.

다른 프로그래밍 언어를 안다면 주의하세요. Lua에서는 `0`과 빈 문자열 `""`도 참으로 간주됩니다!

<div class="runner">

```lua
if 0 then
    print("0은 참으로 간주됩니다")
end

if "" then
    print("빈 문자열은 참으로 간주됩니다")
end

if nil then
    print("이것은 절대 출력되지 않습니다")
end
```

</div>

## `== nil` 없이 `nil` 확인하기

이 규칙 덕분에 변수를 그대로 조건으로 사용해 값이 있는지 확인할 수 있습니다.

<div class="runner">

```lua
local name = nil

if name then
    print("안녕하세요 " .. name)
else
    print("이름이 없습니다")
end
```

</div>

`if name then`은 `if name ~= nil then`의 짧은 표현이고, `if not name then`은 `if name == nil then`의 짧은 표현입니다.

<div class="note">

변수가 `false`를 담을 수 있을 때 이 둘은 정확히 같지 않습니다. `false`는 `nil`이 아니지만 조건에서는 거짓으로 간주됩니다.

</div>

## `not`

`not` 키워드는 항상 진짜 부울 값을 돌려줍니다. `false`와 `nil`에는 `true`를, 그 외의 모든 것에는 `false`를 돌려줍니다.

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or`와 `and`는 값 중 하나를 돌려줍니다

`or`와 `and`를 부울 값으로 보았지만, 이들은 어떤 값에서도 작동하며 항상 부울 값을 돌려주지는 않습니다. 두 값 중 하나를 돌려줍니다.

- `a or b`는 `a`가 참으로 간주되면 `a`를, 아니면 `b`를 돌려줍니다.
- `a and b`는 `a`가 거짓으로 간주되면 `a`를, 아니면 `b`를 돌려줍니다.

<div class="runner">

```lua
print(nil or "기본값")
print("값" or "기본값")

print(nil and "다음")
print("값" and "다음")
```

</div>

`true`와 `false`의 경우, 이것은 이전에 본 표와 정확히 같습니다.

## `or`로 기본값 설정하기

이것이 `or`가 변수에 기본값을 줄 수 있는 이유입니다. 선택적 인수에서 했던 것과 같습니다.

<div class="runner">

```lua
local name = nil
name = name or "게스트"

print(name)
```

</div>

`name`에 값이 있으면 유지됩니다. `name`이 `nil`이면 `"게스트"`가 됩니다.

## 두 값 중 하나 고르기

`and`와 `or`를 함께 사용하면 조건에 따라 두 값 중 하나를 한 줄로 고를 수 있습니다.

<div class="runner">

```lua
local score = 70
local result = score > 50 and "당신이 이겼습니다!" or "당신이 졌습니다!"

print(result)
```

</div>

이것은 다음과 같습니다.

```lua
local result
if score > 50 then
    result = "당신이 이겼습니다!"
else
    result = "당신이 졌습니다!"
end
```

<div class="note">

첫 번째 값이 `false` 또는 `nil`이면 이 트릭은 작동하지 않습니다. `condition and false or "다른 값"`은 항상 `"다른 값"`을 돌려줍니다. 이 경우에는 일반 `if`를 사용하세요.

</div>

## `and`는 가능한 한 빨리 멈춥니다

`and`의 첫 번째 값이 거짓으로 간주되면 Lua는 두 번째 값을 보지도 않습니다. 코스 초반에 `nil`과 숫자를 비교했을 때 발생한 오류를 기억하나요? `and`는 그것을 피할 수 있게 해 줍니다.

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("추측값이 10보다 큽니다")
end

print("오류 없음!")
```

</div>

`user_guess ~= nil`이 `false`이므로 `user_guess > 10`은 절대 실행되지 않습니다. 마찬가지로 `or`는 첫 번째 값이 참으로 간주되면 두 번째 값을 보지 않습니다.

## 테이블에서의 `nil`

테이블에 존재하지 않는 키는 오류 없이 `nil`을 돌려줍니다. 그리고 테이블에서 키를 제거하려면 그것에 `nil`을 할당합니다.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

이것은 `score` 같은 키를 제거하는 올바른 방법이지만, 리스트의 값에는 사용할 수 없습니다. 우리가 본 것처럼 구멍이 생깁니다. 리스트에는 `table.remove`를 사용하세요.

</div>
