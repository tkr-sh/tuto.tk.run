지금까지 우리는 다음 문법으로만 Lua 변수를 선언했습니다.

```lua
my_variable = "value"
```

그리고 우리가 본 것처럼 그것은 분명히 작동합니다. 하지만 항상 최선의 방법은 아닐 수 있습니다.
다음 시나리오를 봅시다.

<div class="runner">

```lua
function my_function(a)
    b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```
</div>

`b`는 여전히 함수 _외부_ 에 정의되어 있습니다. 그것을 원할 수도 있습니다. 하지만 대부분의 경우 예상된 동작이 아닙니다. 대부분의 경우 함수 안, 루프 안, 단일 범위 안에만 존재하는 변수를 만들고 싶습니다.

값이 특정 범위 안에만 존재하도록 하려면 변수 이름 앞에 `local` 키워드를 붙이면 됩니다.

```lua
local my_variable = "value"
```

이렇게 하면 변수는 정의된 블록(`end`, `repeat ... until` 등)의 끝에 도달할 때까지, 또는 블록 외부에서 정의된 경우 파일 끝까지 존재합니다. 이것으로 예제를 고칠 수 있습니다.

<div class="runner">

```lua
function my_function(a)
    local b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```

</div>

<div class="note">

`for` 값은 기본적으로 `local`입니다.

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

변수를 선언하는 것과 변수에 값을 할당하는 것은 서로 다른 일입니다.

- **선언**은 `local`의 범위를 가진 변수를 만듭니다: `local my_variable`
- **할당**은 변수에 값을 줍니다: `my_variable = "value"`

`local my_variable = "value"`라고 쓰면 단순히 이 두 가지를 한 번에 하는 것입니다.

값을 주지 않고 변수를 선언할 수도 있습니다. 무언가를 할당할 때까지 `nil`을 담고 있습니다.

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

변수마다 `local`은 한 번만 쓰세요. 두 번째 `local user_guess = 42`는 첫 번째 변수를 바꾸는 대신 (아마도 새로운 범위에) 새로운 변수를 만듭니다.

</div>

그리고 이 마지막 예제로 게임을 완성할 수 있습니다.

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    local user_input

    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        local is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

local scores = {}

repeat 
	local number_of_tries = 0
	local number_to_guess = math.random(1, 100)

	print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")

	repeat
        local user_guess = validated_read(
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

    scores[#scores + 1] = number_of_tries

    local user_answer = validated_read(
        "계속하시겠습니까? yes / no",
        "유효한 답변이 아닙니다.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("점수:")

for game_number, score in ipairs(scores) do
    print("게임 " .. game_number .. ": " .. score .. "번 만에 성공!")
end

print("안녕히 가세요!")
```

</div>
