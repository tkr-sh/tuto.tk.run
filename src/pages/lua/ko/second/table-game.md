이제 이 모든 것을 알았으니 게임 기록을 작동시킬 수 있습니다!

점수 기록을 만들려면 프로그램 맨 처음에 새 테이블 `scores`를 만들고, 게임을 끝낼 때마다 `number_of_tries`를 `scores[#scores + 1]`, 즉 마지막 요소로 추가하면 됩니다.

게임이 끝나면 `ipairs`로 테이블을 순회하며 모든 점수를 출력할 수 있습니다!

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

scores = {}

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

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
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
