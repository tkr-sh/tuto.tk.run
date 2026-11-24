## 인라인 함수

지금까지 우리는 함수를 이렇게 작성했습니다.

```lua
function double(number)
    return number * 2
end
```

이것은 설탕입니다. Lua가 실제로 하는 일은 이름 없는 함수를 만들어 `double`이라는 변수에 저장하는 것입니다.

```lua
double = function(number)
    return number * 2
end
```

두 버전은 정확히 같습니다. 이것이 `tonumber`를 `validated_read`에 전달할 수 있었던 이유이기도 합니다. 함수는 숫자나 문자열처럼 변수에 저장된 값일 뿐입니다.


## `:`로 함수 호출하기

문자열을 소문자로 만들기 위해 우리는 `string.lower(user_answer)`라고 썼습니다.
그런데 `string` 또는 `table` 라이브러리의 함수 중에서 첫 번째 인수로 `string` 또는 `table`을 받는 함수를 사용할 때는 다음 문법으로 쓸 수 있습니다: `user_answer:lower()`.

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

`:`는 왼쪽의 값을 가져와 함수의 첫 번째 인수로 전달합니다. 문자열의 경우 Lua는 `string` 테이블에서 함수를 찾으므로 `answer:lower()`는 `string.lower(answer)`가 됩니다.

<div class="note">

이것은 코드에 직접 쓴 문자열에는 작동하지 않습니다. `"YES":lower()`는 문법 오류입니다. Lua는 변수 이름이나 괄호 뒤의 `:`만 허용하기 때문입니다. 작동하게 하려면 문자열을 괄호로 감싸세요.

```lua
print(("YES"):lower())
-- 이것은 오류가 납니다:
print("YES":lower())
```

</div>

## 괄호 없이

함수를 하나의 인수로 호출하고 그 인수가 코드에 직접 쓴 `string` 또는 `table`일 때는 괄호를 생략할 수 있습니다.

<div class="runner">

```lua
print "Hello!"
print("Hello!")
```

</div>

다른 사람의 코드에서 이것을 보게 될 것입니다. 이 코스에서는 괄호를 유지합니다. 함수가 호출되고 있다는 것이 분명해지기 때문입니다.

## 여러 값 반환하기

함수는 하나 이상의 값을 반환할 수 있습니다. `return` 뒤에 `,`로 구분하고, 여러 변수로 받습니다. 이때도 `,`로 구분합니다.

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

변수의 수가 값보다 적으면 남는 값은 그냥 버려집니다. 변수의 수가 더 많으면 남는 변수는 `nil`이 됩니다.

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

`function`의 첫 번째 값만 유지하려면 호출을 괄호로 감싸세요.

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## 부족한 인수와 남는 인수

Lua는 `function`에 전달하는 인수의 수를 확인하지 않습니다. 부족한 인수는 `nil`이고, 예상하지 않은 인수는 무시됩니다.

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## 선택적 인수

부족한 인수는 `nil`이므로 그것을 확인하고 대신 기본값을 사용할 수 있습니다. 이렇게 하면 인수가 선택적이 됩니다.

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Hello"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Good evening")
```
