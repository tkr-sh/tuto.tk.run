أول شيء قد تكون لاحظته هو أن لعبتنا حاليًّا *تتعطّل* عندما ندخل رقمًا ليس رقمًا صالحًا:

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("الرقم أقل!")
end
```

</div>

وهو ما سيعطيك، إذا لم تكتب رقمًا:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

لكن ماذا يعني هذا؟

المفهوم الأساسي هنا هو `nil`. أثناء تقديم أنواع Lua، رأينا بإيجاز نوع `nil`:

> `nil`: فارغ. الهدف من `nil` هو الإشارة إلى أنك لا تملك قيمة.

<div style="height: 3px"></div>

وهذا _بالضبط_ ما تفعله `tonumber`:

- إذا كان `io.read()` سلسلة نصية تمثّل رقمًا مثل `"3.14"` أو `"42"`، فتعرف Lua كيف تحوّلها من نوع `string` إلى نوع `number`.
- وإلا، إذا كان `io.read()` سلسلة نصية لا تشبه `number`، فتقول `tonumber` فقط "لا أعرف كيف أحوّل هذا إلى رقم، فلنُرجع `nil`".

لذلك، إذا لم نمرّر في `io.read()` تمثيلًا صحيحًا لرقم، فسيكون لدينا فقط `nil`:

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## حسنًا، لكن لماذا يُنشئ خطأً إذن؟

سؤال جيد! في الوقت الحالي، `user_guess` هو مجرّد `nil`. يحدث الخطأ عندما نحاول مقارنة `number` بـ `nil` باستخدام العامل `<` في هذا السطر:

```lua
if 10 < user_guess then
```

والخطأ موصوف بشكل جيد في رسالة الخطأ:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` يعني السطر الذي يحدث فيه الخطأ، في حالتنا، سطر المقارنة لدينا.
- `attempt to compare number with nil` هو سبب حدوث الخطأ.

وهذا منطقي تمامًا! ماذا يعني أن تقارن، من حيث الأكبر أو الأصغر، لا شيء برقم؟ هل هي `false`؟ هل هي `true`؟ لا، السؤال ببساطة لا معنى له. لذا تُصدر Lua خطأً فقط.

## معالجة الخطأ
أسرع طريقة لمعالجة هذا هي فقط التحقّق مما إذا كان `user_guess` هو `nil` وهذا يمكن فعله باستخدام العامل `==`:

```lua
if user_guess == nil then
	print("هذا ليس رقمًا صالحًا")
end
```

لكن في هذه الحالة، لا نريد فقط طباعة شيء ومتابعة الحلقة لأنه سيُحتسب كتخمين بسبب:

```lua
number_of_tries = number_of_tries + 1
```

طريقة سهلة للقيام بذلك هي طلب `user_guess` حتى لا يكون `nil` باستخدام العامل `~=`، أي حتى يكون `number` صالحًا:

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

يمكننا أيضًا إضافة رسالة خطأ في حال كان كذلك باستخدام:

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("هذا ليس رقمًا صالحًا!")
	end
until user_guess ~= nil
```

وهو ما يعطينا هذا البرنامج الكامل:


<div class="runner">

```lua
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
```

</div>
