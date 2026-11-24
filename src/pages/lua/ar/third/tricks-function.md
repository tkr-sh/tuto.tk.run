## الدوال المضمّنة (Inline)

حتى الآن، كتبنا الدوال هكذا:

```lua
function double(number)
    return number * 2
end
```

هذا سكر. ما تفعله Lua حقًّا هو إنشاء دالة بدون اسم وتخزينها في متغيّر يُسمّى `double`:

```lua
double = function(number)
    return number * 2
end
```

النسختان متطابقتان تمامًا. هذا أيضًا هو سبب تمكّننا من إعطاء `tonumber` إلى `validated_read`: فالدالة مجرّد قيمة مخزّنة في متغيّر، مثل رقم أو سلسلة نصية.


## استدعاء الدوال باستخدام `:`

لوضع سلسلة نصية بأحرف صغيرة، كتبنا `string.lower(user_answer)`.
لكن، عندما تستخدم دالة من مكتبة `string` أو `table`، تأخذ كوسيط أول `string` أو `table`، يمكنك كتابة هذا بالصياغة التالية: `user_answer:lower()`:

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

تأخذ `:` القيمة على يسارها وتعطيها للدالة كوسيطها الأول. بالنسبة لسلسلة نصية، تبحث Lua عن الدالة في جدول `string`، لذا يصبح `answer:lower()` هو `string.lower(answer)`.

<div class="note">

هذا لا يعمل على سلسلة نصية مكتوبة مباشرةً في الشيفرة: فـ `"YES":lower()` خطأ نحوي، لأن Lua تقبل `:` فقط بعد اسم متغيّر أو بعد أقواس. لجعله يعمل، ضع السلسلة النصية بين أقواس:

```lua
print(("YES"):lower())
-- هذا يُصدر خطأ:
print("YES":lower())
```

</div>

## بدون أقواس

عندما تُستدعى دالة بوسيط واحد، وهذا الوسيط هو `string` أو `table` مكتوب مباشرةً في الشيفرة، فإن الأقواس اختيارية:

<div class="runner">

```lua
print "Hello!"
print("Hello!")
```

</div>

سترى هذا في شيفرة الآخرين. في هذا الدرس نحتفظ بالأقواس، لأنها تجعل من الواضح أن دالة تُستدعى.

## إرجاع عدة قيم

يمكن للدالة أن تُرجع أكثر من قيمة. تفصل بينها بـ `,` بعد `return`، وتستقبلها في عدّة متغيّرات، مفصولةً أيضًا بـ `,`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

إذا أعطيت متغيّرات أقل ممّا هناك قيم، فببساطة تُهمل القيم الإضافية. وإذا أعطيت أكثر، فالمتغيّرات الإضافية تكون `nil`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

للاحتفاظ بالقيمة الأولى فقط من `function`، ضع الاستدعاء بين أقواس:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## الوسائط المفقودة والإضافية

لا تتحقّق Lua من عدد الوسائط التي تعطيها لـ `function`. الوسيط المفقود هو `nil`، والوسيط غير المتوقّع يُهمل:

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## الوسائط الاختيارية

بما أن الوسيط المفقود هو `nil`، يمكننا التحقّق منه واستخدام قيمة افتراضية بدلًا منه. هذا يجعل الوسيط اختياريًّا:

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Hello"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Good evening")
```
