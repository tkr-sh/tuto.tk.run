입력 검증을 위해 비슷한 코드 블록을 두 번 작성한다는 것을 눈치챘을 수도 있습니다.

```lua
repeat
    print("숫자를 맞혀 보세요:")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("유효한 숫자가 아닙니다!")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("계속하시겠습니까? yes / no")
    user_answer = string.lower(io.read())
    if user_answer ~= "yes" and user_answer ~= "no" then
        print("유효한 답변이 아닙니다.")
    end
until user_answer == "yes" or user_answer == "no"
```

명백한 공통 중복 패턴이 있습니다. 이를 `<`와 `>`로 구분한 의사/잘못된 문법으로 보여 드리겠습니다.

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## 함수의 기초

프로그램을 작성할 때 자주 따라야 하는 규칙이 있습니다. 바로 모듈식으로 작성하는 것입니다. 그래야 나중에 유용하게 쓰일 수 있고, 대부분의 경우를 처리할 만큼 일반적일 수 있습니다. 함수는 바로 그것을 가능하게 도와줍니다.

함수는 기본적으로 프로그램이 수행할 명령어의 집합입니다. 함수는 매개변수를 받을 수 있고 값을 반환할 수 있습니다. 예를 들어 이 함수를 봅시다.

```lua
function double(my_number)
    return my_number * 2
end
```

이 함수의 이름은 `double`이고, `my_number`라는 하나의 매개변수를 받습니다. 이 함수의 결과, 즉 반환하는 값은 `my_number * 2`라는 하나의 값입니다. 함수는 이름을 지정하고 괄호 안에 `,`로 구분한 인수를 넣어 호출할 수 있습니다. 예를 들어 `my_function(value1, value2)`처럼요. 우리의 경우:

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## 게임으로 돌아가서!

그럼 입력 처리를 최적화해 봅시다! 먼저 우리 함수가 무엇에 의존하는지 살펴봐야 합니다. 어떤 인수가 필요한지 알려면 이것이 필요합니다. 우리의 경우:
* 프롬프트 메시지, 사용자에게 값을 입력하라고 말하는 메시지
* 오류 메시지, 입력이 유효하지 않을 때 보여 줄 메시지
* 변환, 사용자로부터 읽은 문자열에 적용하는 변환(`tonumber`, `string.lower`, `string.upper` 등)
* 조건, 변환을 적용한 후 입력이 우리가 기대하는 것과 맞는지 확인하는 검사.

`prompt_message`와 `error_message`는 비교적 간단합니다. 그냥 문자열입니다.<br/>

`transform`과 `condition`은 조금 더 복잡합니다.<br>
`transform`은 사용자 입력을 받아 수정된 버전을 돌려줍니다. 이는 분명히 함수의 경우입니다. 무언가를 입력으로 받아 무언가를 돌려주는 것이니까요.<br/>
`condition`도 정말 비슷합니다. `transform`의 결과를 인수로 받아 결과에 따라 부울 값(`true`/`false`)을 반환합니다. 이 역시 분명히 `function`이 필요한 경우입니다.

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end
```

`not`을 주목하세요! `not` 키워드는 부울 값을 뒤집습니다. `true` => `false`, `false` => `true`입니다.

우리의 경우, 사용자로부터 숫자를 받는 입력 처리를 이렇게 작성할 수 있습니다.

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "숫자를 맞혀 보세요:",
    "유효한 숫자가 아닙니다",
    tonumber,
    is_not_nil
)
```

이것은 다음과 같은 결과를 냅니다.

```lua
repeat
    print("숫자를 맞혀 보세요:")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("유효한 숫자가 아닙니다!")
    end
until is_valid
```

## 함수 인라인화

`is_not_nil`을 한 번만 사용하려고 정의했다는 것을 주목하세요. 다른 곳에서 사용할 필요가 없을 때 이름이 있는 함수를 선언하는 대신, Lua에서는 다음과 같이 이름을 없앨 수 있습니다.

```lua
local double = function(number)
    return number * 2
end
```

우리의 경우, 이것을 사용해 이렇게 할 수 있습니다.

```lua
validated_read(
    "숫자를 맞혀 보세요:",
    "유효한 숫자가 아닙니다",
    tonumber,
    function(nb) return nb ~= nil end
)
```

그러면 최종 프로그램이 됩니다.

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")

	repeat
        user_guess = validated_read(
            "1에서 100 사이의 숫자를 입력하세요:",
            "유효한 숫자가 아닙니다",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("숫자가 더 작습니다!")
		end

		if number_to_guess > user_guess then
			print("숫자가 더 큽니다!")
		end
	until user_guess == number_to_guess

	print("당신은 " .. number_of_tries .. "번 만에 맞혔습니다!")

    user_answer = validated_read(
        "계속하시겠습니까? yes / no",
        "유효한 답변이 아닙니다.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("안녕히 가세요!")
```

</div>
