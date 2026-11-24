## ما الذي يُحتسب على أنه `true`؟

في شرط ما، لا تقبل Lua `true` و`false` فقط: يمكن استخدام أي قيمة. القاعدة بسيطة: `false` و`nil` يُحتسبان على أنهما خطأ (false)، و**كل شيء آخر** يُحتسب على أنه صحيح (true).

كن حذرًا إذا كنت تعرف لغة برمجة أخرى: في Lua، يُحتسب `0` والسلسلة الفارغة `""` على أنهما صحيح (true)!

<div class="runner">

```lua
if 0 then
    print("0 يُحتسب على أنه صحيح")
end

if "" then
    print("السلسلة الفارغة تُحتسب على أنها صحيحة")
end

if nil then
    print("هذا لا يُطبع أبدًا")
end
```

</div>

## التحقّق من `nil` دون `== nil`

بفضل هذه القاعدة، يمكننا التحقّق من أن متغيّرًا لديه قيمة باستخدامه مباشرةً كشرط:

<div class="runner">

```lua
local name = nil

if name then
    print("مرحبًا " .. name)
else
    print("لا يوجد اسم")
end
```

</div>

`if name then` طريقة أقصر لكتابة `if name ~= nil then`، و`if not name then` طريقة أقصر لكتابة `if name == nil then`.

<div class="note">

هذه ليست متطابقة تمامًا عندما يمكن أن يحتوي المتغيّر على `false`: فـ `false` ليس `nil`، لكنه أيضًا يُحتسب على أنه خطأ في شرط ما.

</div>

## `not`

الكلمة المفتاحية `not` تُعطي دائمًا boolean حقيقيًّا. تُعطي `true` مقابل `false` و`nil`، و`false` مقابل كل شيء آخر:

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or` و`and` يُعيدان إحدى قيمتيهما

رأينا `or` و`and` مع القيم المنطقية (boolean)، لكنهما يعملان مع أي قيمة، ولا يُعيدان دائمًا boolean: بل يُعيدان إحدى قيمتيهما.

- `a or b` يُعطي `a` إذا كان يُحتسب على أنه صحيح، و`b` خلاف ذلك.
- `a and b` يُعطي `a` إذا كان يُحتسب على أنه خطأ، و`b` خلاف ذلك.

<div class="runner">

```lua
print(nil or "default")
print("value" or "default")

print(nil and "next")
print("value" and "next")
```

</div>

مع `true` و`false`، يعطي هذا بالضبط الجداول التي رأيناها سابقًا.

## القيم الافتراضية باستخدام `or`

هذا هو سبب قدرة `or` على إعطاء قيمة افتراضية لمتغيّر، كما فعلنا مع الوسائط الاختيارية:

<div class="runner">

```lua
local name = nil
name = name or "Guest"

print(name)
```

</div>

إذا كان لـ `name` قيمة، فسيُحتفظ بها. وإذا كان `name` هو `nil`، فسيصبح `"Guest"`.

## الاختيار بين قيمتين

باستخدام `and` و`or` معًا، يمكننا الاختيار بين قيمتين بشرط، في سطر واحد:

<div class="runner">

```lua
local score = 70
local result = score > 50 and "لقد فزت!" or "لقد خسرت!"

print(result)
```

</div>

هذا هو نفس:

```lua
local result
if score > 50 then
    result = "لقد فزت!"
else
    result = "لقد خسرت!"
end
```

<div class="note">

هذه الحيلة لا تعمل إذا كانت القيمة الأولى `false` أو `nil`: فـ `condition and false or "other"` يُعطي دائمًا `"other"`. في هذه الحالة، استخدم `if` عادية.

</div>

## `and` يتوقّف بأسرع ما يمكن

عندما تُحتسب القيمة الأولى في `and` على أنها خطأ، لا تنظر Lua حتى إلى الثانية. تذكّر الخطأ الذي واجهناه في بداية الدرس عند مقارنة `nil` برقم؟ يتيح لنا `and` تجنّبه:

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("التخمين أكبر من 10")
end

print("لا خطأ!")
```

</div>

بما أن `user_guess ~= nil` هو `false`، فإن `user_guess > 10` لا يُنفّذ أبدًا. وبنفس الطريقة، لا ينظر `or` إلى قيمته الثانية عندما تُحتسب الأولى على أنها صحيحة.

## `nil` في الجداول

المفتاح غير الموجود في جدول يُعطي `nil`، دون أي خطأ. ولإزالة مفتاح من جدول، تُسند إليه `nil`:

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

هذه هي الطريقة الصحيحة لإزالة مفتاح مثل `score`، لكن ليس لإزالة قيمة في قائمة: كما رأينا، سيُنشئ ذلك فجوة. بالنسبة للقوائم، استخدم `table.remove`.

</div>
