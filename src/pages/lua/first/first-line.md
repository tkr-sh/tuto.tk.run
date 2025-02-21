
```lua
number_of_tries = 0
```

afds
qf

qdf
qdfsf
sqdf


q
s


df
qdfdsf


qsdfdf



qsdfqs
df



qsdf



qsdfqssdf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf
qdfsff



qsdfsddsf
divf


qdfsfqdsf


<button id="toggle-code"/>

<div id="hidden-code">

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("Try to guess the random number between 1 and 100!")

repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("The number is lower!")
	end

	if number_to_guess > user_guess then
		print("The number is higher!")
	end
until user_guess == number_to_guess

print("You won in " .. number_of_tries .. " tries!")
```

</div>
