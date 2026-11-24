الآن بعد أن عرفنا كل ذلك، يمكننا جعل سجلّ ألعابنا يعمل!

لإنشاء سجلّ لنتائجنا، يمكننا إنشاء جدول جديد `scores` في بداية برنامجنا تمامًا، وفي كل مرة ننهي لعبة، نضيف `number_of_tries` إليه كـ `scores[#scores + 1]`، أي العنصر الأخير.

بمجرّد انتهاء اللعبة، يمكننا ببساطة المرور عبر الجدول باستخدام `ipairs` وطباعة كل نتائجنا!


وهذا يعطينا برنامجنا النهائي:

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

	print("حاول تخمين الرقم العشوائي بين 1 و100!")

	repeat
        user_guess = validated_read(
            "أدخل رقمًا بين 1 و100:",
            "هذا ليس رقمًا صالحًا",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("الرقم أقل!")
		end

		if number_to_guess > user_guess then
			print("الرقم أكبر!")
		end
	until user_guess == number_to_guess

	print("لقد فزت في " .. number_of_tries .. " محاولات!")

    scores[#scores + 1] = number_of_tries

    user_answer = validated_read(
        "هل تريد المتابعة؟ نعم / لا",
        "هذه إجابة غير صالحة.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("النتائج:")

for game_number, score in ipairs(scores) do
    print("اللعبة " .. game_number .. ": انتهت في " .. score .. " محاولات!")
end

print("وداعًا!")
```

</div>
