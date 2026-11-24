Ĉi tiu estas la unua programo, kiun ni studos! Ne zorgu, ni trairos ĝin TRE malrapide, do estas en ordo, se vi ankoraŭ komprenas nenion!

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("Provu diveni la hazardan nombron inter 1 kaj 100!")

repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("La nombro estas pli malgranda!")
	end

	if number_to_guess > user_guess then
		print("La nombro estas pli granda!")
	end
until user_guess == number_to_guess

print("Vi venkis en " .. number_of_tries .. " provoj!")
```

Kion ĉi tiu programo faras estas krei la ludon "trovu la nombron":

```
Provu diveni la hazardan nombron inter 1 kaj 100!
50
La nombro estas pli granda!
75
La nombro estas pli malgranda!
62
Vi venkis en 3 provoj!
```
