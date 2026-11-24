Ĝis nun, ni nur deklaris Lua-variablojn per ĉi tiu sintakso:

```lua
my_variable = "value"
```

Kaj ĝi funkcias, klare, kiel ni vidis. Sed ĝi eble ne ĉiam estas la plej bona maniero fari tion.
Ni prenu ĉi tiun scenaron:

<div class="runner">

```lua
function my_function(a)
    b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```
</div>

`b` estas ankoraŭ difinita _ekster_ la funkcio. Kion vi eble volas. Sed plejofte, tio ne estas atendita konduto. Plejofte, vi volas krei variablon, kiu ekzistas nur en funkcio, en buklo, en unuopa amplekso.

Por difini valoron tiel, ke ĝi estu nur en speciala amplekso, ni povas simple meti la ŝlosilvorton `local` antaŭ la nomo de la variablo tiel:

```lua
local my_variable = "value"
```

Per ĉi tio, la variablo ekzistos ĝis ĝi atingos aŭ la finon de la bloko (`end`, `repeat ... until`, ...) en kiu ĝi estis difinita, aŭ la finon de la dosiero se ĝi estis difinita ekster bloko. Per ĉi tio, ni povas ripari nian ekzemplon:

<div class="runner">

```lua
function my_function(a)
    local b = a * 2
    return a + b
end

print(my_function(2))
print(b)
```

</div>

<div class="note">

`for`-valoroj estas `local` defaŭlte

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

Deklari variablon kaj asigni valoron al ĝi estas du malsamaj aferoj:

- **Deklari** kreas la variablon kun la amplekso de `local`: `local my_variable`
- **Asigni** donas valoron al variablo: `my_variable = "value"`

Skribi `local my_variable = "value"` simple faras ambaŭ samtempe.

Vi ankaŭ povas deklari variablon sen doni al ĝi valoron. Ĝi entenos `nil` ĝis vi asignos ion al ĝi:

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

Skribu `local` nur unufoje por variablo: dua `local user_guess = 42` kreas novan variablon (eble en nova amplekso) anstataŭ ŝanĝi la unuan.

</div>

Kaj ni povas fini nian ludon per ĉi tiu fina ekzemplo:

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    local user_input

    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        local is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

local scores = {}

repeat 
	local number_of_tries = 0
	local number_to_guess = math.random(1, 100)

	print("Provu diveni la hazardan nombron inter 1 kaj 100!")

	repeat
        local user_guess = validated_read(
            "Enigu nombron inter 1 kaj 100:",
            "Ĉi tio ne estas valida nombro",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("La nombro estas pli malgranda!")
		end

		if number_to_guess > user_guess then
			print("La nombro estas pli granda!")
		end
	until user_guess == number_to_guess

	print("Vi venkis en " .. number_of_tries .. " provoj!")

    scores[#scores + 1] = number_of_tries

    local user_answer = validated_read(
        "Ĉu vi volas daŭrigi? jes / ne",
        "Ĉi tio ne estas valida respondo.",
        string.lower,
        function(response) return response == "jes" or response == "ne" end
    )
until user_answer == "ne"

print("Poentoj:")

for game_number, score in ipairs(scores) do
    print("Ludo " .. game_number .. ": finita en " .. score .. " provoj!")
end

print("Ĝis revido!")
```

</div>
