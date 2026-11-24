Unuafoje, ni ne koncentriĝos pri unu sola linio! La nuna linio estas simple

```lua
repeat
```

sed en si mem, ĝi faras nenion: ĝi iras kun `until`.

```lua
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
```

Sed antaŭ ĉio... kio estas `repeat` kaj `until`, kaj kial ili estas tiel specialaj? Ĉu ili ne estas simple iuj strangaj variabloj?

Vi eble memoras la difinon de variablo

> `the_name_of_the_variable` povas esti io ajn. Krom `true`, `false`, `nil` (kaj aliaj, kiujn ni vidos poste)

Kaj la poste... estas nun!

## Ŝlosilvortoj

`repeat` kaj `until` estas tio, kion ni nomas "ŝlosilvortoj". Ŝlosilvortoj estas specialaj nomoj, kiuj, anstataŭ havi valoron (kiel variabloj) aŭ fari iun komputadon (kiel funkcioj), estas instrukcioj, kiuj diras al Lua fari certajn decidojn.

En nia ekzemplo de `repeat`/`until`, ĉio, kio estas inter `repeat` kaj `until`, estos ripetata ĝis certa kondiĉo plenumiĝos. Kiam la plenumo alvenas al `until`, ĝi kontrolas, ĉu ĝi volas reruli, kaj se jes, la kodo reiras al `repeat` kaj ripetiĝas.

Ĉi tiu speco de konduto - ripeti iun kodon ĝis _io_ - nomiĝas "buklo" kaj estas vere ofta koncepto en programado.

Plejofte, vi volas eliri vian buklon ĉe certa punkto...

### Sed kiel scii, kiam eliri?

Kiam vi uzas buklojn kiel `until`, vi devas doni al ĝi `boolean` (`true` aŭ `false`) por diri al ĝi, kiam ĉesi bukli.

Kaj plejofte, vi faras tion per relativaj operaciiloj:

- `==`: Kontroli, ĉu du valoroj estas egalaj. **NE KONFUZU KUN 1 `=`**. 1 `=` estas por asigni valoron, 2 `=` (`==`) estas por kontroli, ke 2 valoroj estas egalaj. Se ili estas la samaj, la valoro estas `true`, alie `false`
- `~=`: Kontroli, ke du valoroj estas malsamaj. Se ili estas malsamaj, la valoro estas `true`, alie `false`
- `>`: Kontroli, ke la maldekstra valoro estas pli granda ol la dekstra. Se jes, la valoro estas `true`, alie `false`
- `<`: Kontroli, ke la maldekstra valoro estas pli malgranda ol la dekstra. Se jes, la valoro estas `true`, alie `false`
- `>=`: Kontroli, ke la maldekstra valoro estas pli granda ol **AŬ EGALA AL** la dekstra. Se jes, la valoro estas `true`, alie `false`
- `<=`: Kontroli, ke la maldekstra valoro estas pli malgranda ol **AŬ EGALA AL** la dekstra. Se jes, la valoro estas `true`, alie `false`


<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```

</div>

### Specoj de bukloj

- `repeat`/`until`: do ripetos iun kodon ĝis certa kondiĉo plenumiĝos, kaj haltos, kiam tiu kondiĉo plenumiĝos

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while`: Bukloj `while` tre similas al la buklo `repeat`/`until`, sed inverse! Vi buklas dum la kondiĉo plenumiĝas! Post la kondiĉo, vi uzas la ŝlosilvorton `do` por komenci plenumi la kodon, kaj vi rebuklas al la komenco, kiam vi vidas la ŝlosilvorton `end`. En plej multaj situacioj, kie vi povas uzi `repeat`, vi ankaŭ povas uzi `while`. Ĝi estas ĉefe persona prefero.

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for`: La lasta buklo estas la buklo `for`, kaj ĝi estas iom malsama, ĉar vi jam scias, kiom da fojoj vi volas ripeti. Kaj vi difinas, de kiu nombro ĝis kiu nombro vi volas ripeti. Fine de ripeto, ĝi aŭtomate pliigos `age` je `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>


## Reen al nia kazo

En nia kazo, ni volas daŭre demandi la uzanton pri nombro ĝis ĝi trovos la ĝustan. Tial ni 
```lua
repeat
    ...
until user_guess == number_to_guess
```
