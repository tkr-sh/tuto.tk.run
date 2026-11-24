الآن، ماذا لو كان بإمكاننا إجراء عدّة جلسات لعب في تنفيذ واحد، وترك المستخدم يقرّر متى يتوقّف؟

لجعل اللعبة قابلة للتكرار، نغلّف البرنامج فقط في حلقة `repeat ... until` أخرى هكذا:

```lua
repeat
	...
	print("لقد فزت في ... محاولات!")

	repeat 
		print("هل تريد المتابعة؟ نعم أو لا")
		user_answer = io.read()
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("وداعًا!")
```

نستخدم كلمة مفتاحية خاصة `or`، وهي تدمج شرطين وتكون `true` إذا كان أحدهما على الأقل صحيحًا.

لنفترض على سبيل المثال أن لدينا boolean اسمه `a` يمكن أن يكون `true` أو `false`، وboolean آخر اسمه `b` يمكن أن يكون أيضًا `true` أو `false`. لذا سيكون التعبير `a or b`:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

يكون `a or b` هو `false` فقط عندما يكون `a` و`b` كلاهما `false`.
هذا عكس العامل `and` الذي يريد أن يكون `a` و`b` كلاهما `true` حتى يكون `true`

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## حالة الأحرف

في Lua، سلسلة نصية بأحرف كبيرة مثل `"A"` و`"B"` و`"C"`، ... ليست مثل `"a"` و`"b"` و`"c"`، .... هذا يعني أنه إذا كتب المستخدم `"Yes"` فلن تعمل وسيُعيد المستخدم حلقة `repeat` رغم أن المستخدم كتب `"Yes"`. هذا على الأرجح ليس ما نريده...

لحسن الحظ، لدى Lua دوال تسمح لنا بتحويل سلسلة نصية إلى أحرف كبيرة كلها أو أحرف صغيرة كلها: `string.upper` و`string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

وهو ما يسمح لنا بفعل التالي

<div class="runner">

```lua
repeat 
	print("هل تريد المتابعة؟ نعم / لا")
	user_answer = string.lower(io.read())
	if user_answer ~= "yes" and user_answer ~= "no" then
		print("هذه إجابة غير صالحة.")
	end
until user_answer == "yes" or user_answer == "no"
```

</div>

بهذا، سيكون `user_answer` دائمًا بأحرف صغيرة بسبب `string.lower()`.
نضيف أيضًا تحقّقًا فقط لطباعة رسالة إذا كانت الإجابة ليست `"yes"` ولا `"no"`.
وهو ما يعطينا هذا البرنامج الكامل:

<div class="runner">

```lua
repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("حاول تخمين الرقم العشوائي بين 1 و100!")

	repeat
		repeat
			user_guess = tonumber(io.read())
			if user_guess == nil then 
				print("هذا ليس رقمًا صالحًا!")
			end
		until user_guess ~= nil
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("الرقم أقل!")
		end

		if number_to_guess > user_guess then
			print("الرقم أكبر!")
		end
	until user_guess == number_to_guess

	print("لقد فزت في " .. number_of_tries .. " محاولات!")

	repeat 
		print("هل تريد المتابعة؟ نعم / لا")
		user_answer = string.lower(io.read())
		if user_answer ~= "yes" and user_answer ~= "no" then
			print("هذه إجابة غير صالحة.")
		end
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("وداعًا!")
```

</div>
