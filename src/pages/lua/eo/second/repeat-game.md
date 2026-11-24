Nun, kio se ni povus havi plurajn ludsesiojn en unu plenumo, kaj lasi la uzanton decidi, kiam ĉesi?

Por igi la ludon ripetebla, ni simple envolvas la programon en alian buklon `repeat ... until` tiel:

```lua
repeat
	...
	print("Vi venkis en ... provoj!")

	repeat 
		print("Ĉu vi volas daŭrigi? jes AŬ ne")
		user_answer = io.read()
	until user_answer == "jes" or user_answer == "ne"
until user_answer == "ne"

print("Ĝis revido!")
```

Ni uzas specialan ŝlosilvorton `or`, kiu kombinas du kondiĉojn kaj estas `true` se almenaŭ unu el ili estas vera.

Ni diru ekzemple, ke ni havas bulean `a`, kiu povas esti `true` aŭ `false`, kaj alian bulean `b`, kiu ankaŭ povas esti `true` aŭ `false`. La esprimo `a or b` do estos:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` estas `false` nur kiam `a` kaj `b` estas `false`.
Ĉi tio estas la malo de la operaciilo `and`, kiu volas, ke `a` kaj `b` estu `true`, por ke ĝi estu `true`

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## Uskleco

En Lua, ĉeno kun majuskloj kiel `"A"`, `"B"`, `"C"`, ... ne estas la sama kiel `"a"`, `"b"`, `"c"`, .... Tio signifas, ke se uzanto tajpas `"Jes"`, ĝi ne funkcios kaj la uzanto refaros la buklon `repeat` eĉ kvankam la uzanto tajpis `"Jes"`. Ĉi tio probable ne estas tio, kion ni volas...

Bonŝance, Lua havas funkciojn, kiuj permesas al ni konverti ĉenon al ĉiuj majuskloj aŭ ĉiuj minuskloj: `string.upper` kaj `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

Kio permesas al ni fari la jenon

<div class="runner">

```lua
repeat 
	print("Ĉu vi volas daŭrigi? jes / ne")
	user_answer = string.lower(io.read())
	if user_answer ~= "jes" and user_answer ~= "ne" then
		print("Ĉi tio ne estas valida respondo.")
	end
until user_answer == "jes" or user_answer == "ne"
```

</div>

Per ĉi tio, la `user_answer` ĉiam estos minuskla pro `string.lower()`.
Ni ankaŭ aldonas kontrolon nur por presi mesaĝon, se la respondo estas nek `"jes"` nek `"ne"`. 
Kio donas al ni ĉi tiun plenan programon:

<div class="runner">

```lua
repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("Provu diveni la hazardan nombron inter 1 kaj 100!")

	repeat
		repeat
			user_guess = tonumber(io.read())
			if user_guess == nil then 
				print("Ĉi tio ne estas valida nombro!")
			end
		until user_guess ~= nil
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("La nombro estas pli malgranda!")
		end

		if number_to_guess > user_guess then
			print("La nombro estas pli granda!")
		end
	until user_guess == number_to_guess

	print("Vi venkis en " .. number_of_tries .. " provoj!")

	repeat 
		print("Ĉu vi volas daŭrigi? jes / ne")
		user_answer = string.lower(io.read())
		if user_answer ~= "jes" and user_answer ~= "ne" then
			print("Ĉi tio ne estas valida respondo.")
		end
	until user_answer == "jes" or user_answer == "ne"
until user_answer == "ne"

print("Ĝis revido!")
```

</div>
