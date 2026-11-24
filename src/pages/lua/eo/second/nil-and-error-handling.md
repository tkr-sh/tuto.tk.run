La unua afero, kiun vi eble rimarkis, estas ke nia ludo nun *paneas*, kiam ni enigas nombron, kiu ne estas valida nombro: 

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("La nombro estas pli malgranda!")
end
```

</div>

kio, se vi ne tajpas nombron, donos al vi:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

Sed kion tio signifas?

La kernkoncepto ĉi tie estas `nil`. Dum ni prezentis la Lua-tipojn, ni mallonge vidis la tipon `nil`:

> `nil`: Malplena. La celo de `nil` estas diri, ke vi ne havas valoron.

<div style="height: 3px"></div>

Kaj ĝuste tion _precize_ faras `tonumber`:

- Se `io.read()` estas ĉeno, kiu reprezentas nombron kiel `"3.14"` aŭ `"42"`, Lua scias kiel konverti ĝin de la tipo `string` al la tipo `number`.
- Alie, se `io.read()` estas ĉeno, kiu ne aspektas kiel `number`, `tonumber` simple diras "Mi ne scias kiel konverti ĉi tion al nombro, ni simple redonu `nil`".

Tial, se en `io.read()` ni ne pasigas ĝustan reprezenton de nombro, ni simple havos `nil`:

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## Bone, sed kial ĝi tiam kaŭzas eraron?

Bona demando! Nuntempe, `user_guess` estas simple `nil`. La eraro okazas, kiam ni provas per la operaciilo `<` kompari `number` kun `nil` per ĉi tiu linio:

```lua
if 10 < user_guess then
```

Kaj la eraro estas bone priskribita en la erarmesaĝo:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` signifas la linion, ĉe kiu la eraro okazas, en ĉi tiu kazo, nia kompara linio.
- `attempt to compare number with nil` estas la kialo, kial okazis eraro.

Kaj tio havas perfektan sencon! Kion signifas kompari, laŭ pli granda aŭ pli malgranda, nenion kun nombro? Ĉu ĝi estas `false`? Ĉu `true`? Ne, la demando simple havas neniun sencon. Do Lua simple eraras.

## Traktado de la eraro
La plej rapida maniero trakti tion estas simple kontroli, ĉu `user_guess` estas `nil`, kaj tio povas esti farata per la operaciilo `==`:

```lua
if user_guess == nil then
	print("Ĉi tio ne estas valida nombro")
end
```

Sed en ĉi tiu kazo, ni ne volas simple presi ion kaj daŭrigi la buklon, ĉar ĝi kalkulus ĝin kiel divenon pro:

```lua
number_of_tries = number_of_tries + 1
```

Facila maniero fari tion estus demandi `user_guess` ĝis ĝi ne estas `nil` per la operaciilo `~=`, tio signifas, ĝis ĝi estas valida `number`:

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

Ni ankaŭ povas aldoni erarmesaĝon por la kazo, ke ĝi estas, per:

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("Ĉi tio ne estas valida nombro!")
	end
until user_guess ~= nil
```

Kio donas al ni ĉi tiun plenan programon:


<div class="runner">

```lua
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
```

</div>
