## علامات الاقتباس

يمكن كتابة سلسلة نصية بعلامات اقتباس مزدوجة `"..."` أو مفردة `'...'`. كلتاهما تُعطيان السلسلة نفسها بالضبط. هذا مفيد عندما يحتوي نصّك على إحدى العلامتين:

<div class="runner">

```lua
print("It's a nice day")
print('He said "hello"')
```

</div>

## السلاسل النصية متعددة الأسطر

لا يمكنك ببساطة إضافة رجوع إلى سطر جديد في منتصف سلسلة `"..."` هكذا:

<div class="runner">

```lua
print("line 1
line 2")
```
</div>

ببساطة لا تسمح Lua بوجود رجوع إلى سطر جديد هكذا في سلسلة نصية عند كتابتها بـ `"` أو `'`.

لكن! يمكنك فعل ذلك باستبدال `"` أو `'` بـ `[[`/`]]`

<div class="runner">

```lua
print([[line1
line2]])
```

</div>

## الطول

العامل `#` الذي استخدمناه على الجداول يعمل أيضًا على السلاسل النصية:

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
تسمح الدالة `string.sub` بإنشاء `string` جديد، من جزء من هذا الـ `string` بتحديد فهرس البداية والنهاية:

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- نفس
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
تسمح الدالة `string.rep` بتكرار `string` عدّة مرات:

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
تقوم الدالة `string.reverse`، كما يوحي اسمها، بعكس `string`:

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
تستبدل الدالة `string.gsub` `string` بآخر، وتطبع أيضًا كم مرة تمّ الاستبدال:

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

يمكنك التحقّق من كل دوال `string` هنا: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
