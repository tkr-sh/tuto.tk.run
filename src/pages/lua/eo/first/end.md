Do nun la sola afero, kiu restas, estas presi la nombron da provoj.

Antaŭe ni pliigis ĝin tuj post akiri la datumojn de la uzanto per

```lua
number_of_tries = number_of_tries + 1
```

Sed nun ni volus presi gratulan mesaĝon kaj samtempe kiom da provoj estis bezonataj?

Estas tempo lerni la lastan operaciilon antaŭ la fino de ĉi tiu unua ludo:

`..`: Kunigas 2 valorojn. 

<div class="runner">

```lua
print("Pasis " .. 4 .. " jaroj")
```

</div>

Kun tio en la kapo, ni povas krei belan mesaĝon:

```lua
print("Vi venkis en " .. number_of_tries .. " provoj!")
```

Kaj jen FINE nia ludo:

---

<div class="runner">

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

</div>

--- 

## ŬAŬ.

Ŭaŭ! Ĉu ne?

Vi nun pretas fariĝi programisto :)!

## Ĉu ĝi finiĝis?

Jes kaj ne.

Bedaŭrinde, mi tro simpligis unu aferon: kutime vi ne povas plenumi `read` kaj `random` tiel. Lua ŝatas meti enkonstruitajn funkciojn en iujn kategoriojn: `math` (por matematikaj funkcioj), `io` (por enigo/eligo-interagoj), `os` (por interagi kun via operaciumo), ...

Kaj tial, kiam vi volas uzi specifajn funkciojn, kiuj apartenas al kategorio, vi devas voki ilin kiel `category.the_function()`

En nia kazo, `random` apartenas al la kategorio `math`, kaj `read` apartenas al la kategorio `io`. `print` kaj `tonumber` estas mallokaj, do vi ne devas specifi ion ajn

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("Provu diveni la hazardan nombron inter 1 kaj 100!")

repeat
	user_guess = tonumber(io.read())
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

</div>

## Gratulon, vi nun finis vian unuan Lua-programon! 🎉

Nun, kiam vi havas la bazojn, ni provos plibonigi nian nunan ludon por aldoni novajn funkciojn, kaj ni lernos novajn aferojn survoje!
