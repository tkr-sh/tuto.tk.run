## مفاتيح السلاسل النصية

عندما يكون المفتاح `string` يشبه اسم متغيّر، يمكنك إزالة القوسين المعقوفين وعلامات الاقتباس:

<div class="runner">

```lua
local player = { ["name"] = "Bob", ["score"] = 42 }
local same_player = { name = "Bob", score = 42 }

print(player["name"])
print(same_player["name"])
```

</div>

## الوصول إلى مفتاح باستخدام `.`

يوجد الاختصار نفسه لقراءة قيمة أو تغييرها: `player.name` هو نفس `player["name"]`.

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player["name"])
print(player.name)

player.score = 50
print(player.score)
```

</div>

لقد كنت تستخدم هذا منذ الفصل الأول: في `io.read()`، فإن `io` هو `table` مدمج في Lua و`read` هو أحد مفاتيحه!

<div class="note">

`player.key` و`player[key]` ليسا نفس الشيء! الأول يبحث عن المفتاح `"key"`، والثاني يستخدم قيمة المتغيّر `key`:

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }
local key = "score"

print(player.key) -- nil
print(player[key]) -- 42
```
</div>

</div>

## `table.concat`
تدمج الدالة `table.concat` كل قيم قائمة في `string` واحد. يمكنك إعطاؤها فاصلًا يوضع بين القيم. هذا مفيد جدًّا لطباعة قائمة، لأن `print` وحدها لا تُظهر ما بداخل جدول:

<div class="runner">

```lua
local letters = { "a", "b", "c" }

print(letters)
print(table.concat(letters))
print(table.concat(letters, ", "))
```

</div>

## `table.insert`
تضيف الدالة `table.insert` قيمة في نهاية قائمة. إذا أعطيتها موضعًا، تُدرج القيمة هناك وتُزاح القيم التالية بمقدار واحد:

<div class="runner">

```lua
local names = { "bob", "carol" }

table.insert(names, "dave")
print(table.concat(names, ", "))

table.insert(names, 2, "alice")
print(table.concat(names, ", "))
```

</div>

## `table.remove`
تزيل الدالة `table.remove` القيمة عند موضع ما وتُزيح القيم التالية، حتى لا تكون هناك فجوة في القائمة. وبدون موضع، تزيل القيمة الأخيرة. كما تُعيد القيمة التي أُزيلت:

<div class="runner">

```lua
local names = { "bob", "alice", "carol" }

print(table.remove(names, 1))
print(table.concat(names, ", "))

print(table.remove(names))
print(table.concat(names, ", "))
```

</div>

## `table.sort`
ترتّب الدالة `table.sort` قائمة، من أصغر قيمة إلى أكبرها، أو بترتيب أبجدي بالنسبة لـ `string`. إنها تغيّر القائمة نفسها ولا تُنشئ قائمة جديدة:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers)
print(table.concat(numbers, ", "))
```

</div>

للترتيب بترتيب آخر، يمكنك إعطاؤها دالة. تستقبل هذه الدالة قيمتين ويجب أن تُرجع `true` إذا كان ينبغي وضع الأولى قبل الثانية:

<div class="runner">

```lua
local numbers = { 42, 7, 19 }

table.sort(numbers, function(a, b) return a > b end)
print(table.concat(numbers, ", "))
```

</div>

---

يمكنك التحقّق من كل دوال `table` هنا: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
