For now, we only declared Lua variables with this syntax:

```lua
my_variable = "value"
```

And it works, clearly, as we saw it. But, it might not always be the best way to do it.
Let's take this scenario:

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

`b` is still defined _outside_ the function. Which, you could want. But most of the time, is not an expected behaviour. Most of the time, you want to create a variable that only exists in a function, in a loop, in a singular scope.

To define a value to be only present in a special scope, we can simply put a `local` keyword before the name of the variable like so:

```lua
local my_variable = "value"
```

With this, the variable will exist until it reaches either the end of the block (`end`, `repeat ... until`, ...) in which it was defined, or the end of the file if it was defined outside of a block. With this, we can fix our example:

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

`for` values are `local` by default

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

Declaring a variable and assigning a value to it are two different things:

- **Declaring** creates the variable with the scope from `local`: `local my_variable`
- **Assigning** gives a value to a variable: `my_variable = "value"`

Writing `local my_variable = "value"` simply does both at once.

You can also declare a variable without giving it a value. It will contain `nil` until you assign something to it:

```lua
local user_guess
print(user_guess) -- nil

user_guess = 42
print(user_guess) -- 42
```

Only write `local` once per variable: a second `local user_guess = 42` would create a new variable (possibly in a new scope) instead of changing the first one.

</div>

And we can finish our game with this final example:

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

	print("Try to guess the random number between 1 and 100!")

	repeat
        local user_guess = validated_read(
            "Enter a number between 1 and 100:",
            "This is not a valid number",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("The number is lower!")
		end

		if number_to_guess > user_guess then
			print("The number is higher!")
		end
	until user_guess == number_to_guess

	print("You won in " .. number_of_tries .. " tries!")

    scores[#scores + 1] = number_of_tries

    local user_answer = validated_read(
        "Do you want to continue? yes / no",
        "This is not a valid answer.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("Scores:")

for game_number, score in ipairs(scores) do
    print("Game " .. game_number .. ": finished in " .. score .. " tries!")
end

print("Goodbye!")
```

</div>
