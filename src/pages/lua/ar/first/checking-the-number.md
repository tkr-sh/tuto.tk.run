الآن بعد أن عرفنا كيف:

1. نولّد رقمًا عشوائيًّا
2. نحصل على مدخلات المستخدم
3. نكرّر

نحتاج إلى مساعدة المستخدم بإخباره ما إذا كان تخمينه أقل أو أكبر من الرقم السرّي.

لذلك، سنستخدم كلمة مفتاحية جديدة، ستجد نفسك تستخدمها كثيرًا: `if`

ما تفعله `if` هو ببساطة تنفيذ شيفرة اعتمادًا على شرط. تتحقّق الحلقات من شرط للتكرار، وتتحقّق `if` من شرط لتعمل مرة واحدة فقط.

فعلى سبيل المثال، يمكننا التحقّق مما إذا كان المستخدم يُدخل `42`، وإن كان كذلك، يمكننا تحيّته برسالة خاصة.
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("اختيار جيد!")
end
```

</div>

إذن في لعبتنا

```lua
if number_to_guess < user_guess then
    print("الرقم أقل!")
end

if number_to_guess > user_guess then
    print("الرقم أكبر!")
end
```

نتحقّق مما إذا كان الرقم أقل/أكبر، وبناءً على ذلك نطبع رسالة لمساعدة المستخدم.

إذا وجد المستخدم الرقم الصحيح، فستتوقّف الحلقة بسبب:

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("الرقم أقل!")
end

if number_to_guess > user_guess then
    print("الرقم أكبر!")
end

if number_to_guess == user_guess then
    print("صحيح!")
end
```

</div>

وبعد إنجاز ذلك، لم يبقَ علينا سوى صنع نهاية جيدة...
