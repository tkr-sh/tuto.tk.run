إذن، الآن الشيء الوحيد المتبقّي هو طباعة عدد المحاولات.

لقد زدناه سابقًا مباشرةً بعد الحصول على بيانات المستخدم باستخدام

```lua
number_of_tries = number_of_tries + 1
```

لكن الآن، نودّ طباعة رسالة تهنئة، وفي الوقت نفسه، كم محاولة استغرقت؟

حان الوقت لتعلّم العامل الأخير قبل نهاية هذه اللعبة الأولى:

`..`: يدمج قيمتين.

<div class="runner">

```lua
print("لقد مضى " .. 4 .. " سنوات")
```

</div>

بهذا في ذهننا، يمكننا إنشاء رسالة جميلة:

```lua
print("لقد فزت في " .. number_of_tries .. " محاولات!")
```

وها هي أخيرًا لعبتنا:

---

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("حاول تخمين الرقم العشوائي بين 1 و100!")

repeat
	user_guess = tonumber(read())
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

---

## واو.

واو! أليس كذلك؟

أنت الآن جاهز لتصبح مبرمجًا :)!

## هل انتهى الأمر؟

نعم ولا.

للأسف، لقد بسّطت شيئًا مبالغةً: عادةً لا يمكنك تنفيذ `read` و`random` بهذه الطريقة. تحبّ Lua وضع الدوال المدمجة في بعض الفئات: `math` (للدوال الرياضية)، و`io` (لتفاعلات الإدخال / الإخراج)، و`os` (للتفاعل مع نظام التشغيل لديك)، ...

ولذلك، عندما تريد استخدام دوال محدّدة تنتمي إلى فئة ما، تحتاج إلى استدعائها مثل `category.the_function()`

في حالتنا، `random` جزء من فئة `math`، و`read` جزء من فئة `io`. أما `print` و`tonumber` فهما عامّان (global) فلا تحتاج إلى تحديد أي شيء

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("حاول تخمين الرقم العشوائي بين 1 و100!")

repeat
	user_guess = tonumber(io.read())
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

## تهانينا، لقد أنهيت الآن أول برنامج Lua لك! 🎉

الآن بعد أن أصبحت لديك الأساسيات، سنحاول تحسين لعبتنا الحالية لإضافة وظائف جديدة، وسنتعلّم أشياء جديدة في الطريق!
