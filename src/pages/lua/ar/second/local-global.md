حتى الآن، عرّفنا متغيّرات Lua فقط بهذه الصياغة:

```lua
my_variable = "value"
```

وهي تعمل، بوضوح، كما رأيناها. لكنها قد لا تكون دائمًا أفضل طريقة للقيام بذلك.
لنأخذ هذا السيناريو:

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

لا يزال `b` معرّفًا _خارج_ الدالة. وهو ما قد ترغب فيه. لكن في معظم الوقت، ليس سلوكًا متوقّعًا. في معظم الوقت، تريد إنشاء متغيّر موجود فقط داخل دالة، أو داخل حلقة، أو داخل نطاق واحد.

لتعريف قيمة موجودة فقط في نطاق خاص، يمكننا ببساطة وضع الكلمة المفتاحية `local` قبل اسم المتغيّر هكذا:

```lua
local my_variable = "value"
```

بهذا، سيوجد المتغيّر حتى يصل إلى نهاية الكتلة (`end`، `repeat ... until`، ...) التي عُرّف فيها، أو إلى نهاية الملف إذا عُرّف خارج كتلة. بهذا، يمكننا إصلاح مثالنا:

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

قيم `for` تكون `local` افتراضيًّا

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

تعريف متغيّر وإسناد قيمة إليه أمران مختلفان:

- **التعريف** يُنشئ المتغيّر بالنطاق من `local`: `local my_variable`
- **الإسناد** يعطي قيمة لمتغيّر: `my_variable = "value"`

كتابة `local my_variable = "value"` تقوم ببساطة بالأمرين معًا في آنٍ واحد.

يمكنك أيضًا تعريف متغيّر دون إعطائه قيمة. سيحتوي على `nil` حتى تُسند إليه شيئًا:

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

اكتب `local` مرة واحدة فقط لكل متغيّر: فـ `local user_guess = 42` الثانية ستُنشئ متغيّرًا جديدًا (ربما في نطاق جديد) بدلًا من تغيير الأول.

</div>

ويمكننا إنهاء لعبتنا بهذا المثال الأخير:

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

	print("حاول تخمين الرقم العشوائي بين 1 و100!")

	repeat
        local user_guess = validated_read(
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

    local user_answer = validated_read(
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
