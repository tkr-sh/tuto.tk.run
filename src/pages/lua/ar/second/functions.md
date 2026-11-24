قد تكون لاحظت أننا نكتب كتلة شيفرة متشابهة مرتين للتحقّق من المدخلات:

```lua
repeat
    print("حاول تخمين رقم:")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("هذا ليس رقمًا صالحًا!")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("هل تريد المتابعة؟ نعم / لا")
    user_answer = string.lower(io.read())
    if user_answer ~= "yes" and user_answer ~= "no" then
        print("هذه إجابة غير صالحة.")
    end
until user_answer == "yes" or user_answer == "no"
```

هناك نمط مكرّر مشترك واضح سأحاول توضيحه باستخدام صياغة شبه/غير صالحة محدّدة بـ `<` و`>`.

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## أساسيات الدالة

عندما تكتب برنامجًا، غالبًا ما تكون هناك قاعدة يجب اتّباعها هي كتابة الأشياء بطريقة معيارية (modular)، حتى تكون مفيدة في المستقبل وعامّة بما يكفي للتعامل مع معظم الحالات، وهذا بالضبط ما تساعدنا الدوال على تحقيقه.

الدالة هي ببساطة مجموعة من التعليمات التي سيقوم بها البرنامج. يمكن للدالة أن تأخذ معاملات ويمكنها أن تُرجع قيمة. لنأخذ هذه الدالة على سبيل المثال:

```lua
function double(my_number)
    return my_number * 2
end
```

اسم هذه الدالة هو `double` وهي تأخذ معاملًا واحدًا يُسمّى `my_number`. نتيجة هذه الدالة، ما تُرجعه، هي قيمة واحدة: `my_number * 2`. يمكنك استدعاء دالة بتحديد الاسم ثم، بين الأقواس، مفصولةً بـ `,`، الوسائط مثل `my_function(value1, value2)`. في حالتنا:

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## لنعد إلى لعبتنا!

إذن، لنحسّن معالجة مدخلاتنا! أول شيء يجب أن ننظر إليه هو: على ماذا تعتمد دالتنا؟ هذا ضروري لمعرفة أي الوسائط تحتاج. في حالتنا:
* رسالة الطلب، الرسالة التي تخبر المستخدم بإدخال قيمة
* رسالة الخطأ، الرسالة التي تُعرض عندما لا تكون المدخلات صالحة
* التحويل، تحويل نطبّقه على السلسلة التي نقرؤها من المستخدم (`tonumber`، `string.lower`، `string.upper`، ...)
* الشرط، تحقّق لمعرفة ما إذا كانت المدخلات تناسب ما نتوقّعه بعد تطبيق التحويل.

`prompt_message` و`error_message` سهلتان نوعًا ما، فهما مجرّد سلاسل نصية.<br/>

`transform` و`condition` أكثر تعقيدًا قليلًا.<br>
يأخذ `transform` مدخلات المستخدم ويُعيد نسخة معدّلة منها. هذه بوضوح حالة دالة: شيء يأخذ شيئًا كمدخل ويُعيد شيئًا.<br/>
`condition` أيضًا مشابه جدًّا: يأخذ كوسيط نتيجة `transform` ويُرجع `boolean` (`true`/`false`) اعتمادًا على النتيجة. هذه مرة أخرى بوضوح حالة تحتاج إلى `function`.

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

لاحظ `not`! الكلمة المفتاحية `not` تقلب قيمة `boolean`: `true` => `false`، و`false` => `true`.

في حالتنا يمكننا فقط كتابة هذا لمعالجة المدخلات الخاصة بالحصول على الرقم من المستخدم:

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "حاول تخمين رقم:",
    "هذا ليس رقمًا صالحًا",
    tonumber,
    is_not_nil
)
```

سيُنتج هذا نفس الشيء مثل:

```lua
repeat
    print("حاول تخمين رقم:")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("هذا ليس رقمًا صالحًا!")
    end
until is_valid
```

## دوال مُضمّنة (Inlining)

لاحظ أننا عرّفنا `is_not_nil` فقط لاستخدامها مرة واحدة. بدلًا من تعريف دالة باسم عندما لا نحتاجها، تسمح لنا Lua بإزالة الاسم فقط عندما لا نريد استخدامها في مكان آخر، هكذا:

```lua
local double = function(number)
    return number * 2
end
```

في حالتنا، يمكننا استخدامها لفعل:

```lua
validated_read(
    "حاول تخمين رقم:",
    "هذا ليس رقمًا صالحًا",
    tonumber,
    function(nb) return nb ~= nil end
)
```

وهو ما يعطينا برنامجنا النهائي:

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

    user_answer = validated_read(
        "هل تريد المتابعة؟ نعم / لا",
        "هذه إجابة غير صالحة.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("وداعًا!")
```

</div>
