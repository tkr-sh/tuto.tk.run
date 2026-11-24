هذا هو أول برنامج سندرسه! لا تقلق، سنتناوله ببطءٍ شديد، فلا بأس إن لم تفهم أي شيء بعد!

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

ما يفعله هذا البرنامج هو إنشاء لعبة "ابحث عن الرقم":

```
حاول تخمين الرقم العشوائي بين 1 و100!
50
الرقم أكبر!
75
الرقم أقل!
62
لقد فزت في 3 محاولات!
```
