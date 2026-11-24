## المزيد من العوامل

أنت تعرف بالفعل `+` و`-` و`*` و`/`. لدى Lua ثلاثة عوامل أخرى لـ `number`:

|العامل|ما يفعله|
|-|-|
|`//`|يقسّم ويزيل الكسور العشرية|
|`%`|يعطي باقي القسمة|
|`^`|يرفع إلى قوة|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

العامل `%` مفيد جدًّا لمعرفة ما إذا كان `number` زوجيًّا أو فرديًّا: يكون `number` زوجيًّا عندما يكون باقي قسمته على `2` هو `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## أعداد `number` صحيحة وأعداد `number` عشرية

لدى Lua نوعان من `number`: أعداد `number` صحيحة مثل `5`، وأعداد `number` عشرية مثل `5.0` أو `3.14`. معظم الوقت لا تحتاج إلى التفكير في ذلك، لكنه يفسّر سبب طباعة بعض النتائج بـ `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

العامل `/` يُعطي دائمًا `number` عشريًّا، حتى عندما تكون النتيجة صحيحة. ولا يزال `5` و`5.0` متساويين.

<div class="note">

أعداد `number` العشرية ليست دقيقة دائمًا! لا تستطيع أجهزة الحاسوب تخزين `number` مثل `0.1` بشكل مثالي، لذا تُعطي بعض المقارنات نتائج مفاجئة:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

تجنّب استخدام `==` على نتيجة حساب بأعداد `number` عشرية.

</div>

## طرق أخرى لكتابة `number`

يمكن كتابة أعداد `number` الكبيرة جدًّا أو الصغيرة جدًّا باستخدام `e`، والتي تعني "ضرب 10 مرفوعة إلى قوة". يمكنك أيضًا كتابة `number` بـ [النظام الست عشري](https://en.wikipedia.org/wiki/Hexadecimal) بالبدء بـ `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## تحويل `number` والسلاسل النصية

تقوم الدالة `tostring` بتحويل `number` إلى `string`، و`tonumber`، التي استخدمناها بالفعل، تفعل العكس:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

تقوم Lua أيضًا بالتحويل بنفسها عندما تستطيع: فـ `string` يشبه `number` يصبح `number` في حساب ما، ويصبح `number` هو `string` باستخدام `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` و`math.ceil`
تقوم الدالة `math.floor` بتقريب `number` نحو الأسفل، وتقوم `math.ceil` بتقريبه نحو الأعلى:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

للتقريب إلى أقرب `number` صحيح، أضف `0.5` قبل استخدام `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` و`math.min`
تُعطي الدالة `math.max` أكبر الأعداد `number` التي تعطيها لها، و`math.min` أصغرها:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
تزيل الدالة `math.abs` إشارة `number`. هذا مفيد للحصول على المسافة بين عددين من نوع `number`:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
تُعطي الدالة `math.sqrt` الجذر التربيعي لـ `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

يمكنك التحقّق من كل دوال `math` هنا: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
