Now, what if we could have multiple game sessions in one execution, and let the user decide when to stop?

To make the game repeatable we just wrap the program in another `repeat ... until` loop like so:

```lua
repeat
	...
	print("You won in ... tries!")

	repeat 
		print("Do you want to continue? yes OR no")
		user_answer = io.read()
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("Goodbye!")
```

We use a special keyword `or`, that combines two conditions and is `true` if at least one of them is true.

Let's say for example that we have a boolean `a` that can be `true` or `false`, and another boolean `b` that can also be `true` or `false`. The expression `a or b` will therefore be:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` is only `false` when `a` and `b` are `false`.
This is the opposite of the `and` operator that wants `a` and `b` to be `true` for it to be `true`

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## Casing

In Lua, a string with capitalized letters like `"A"`, `"B"`, `"C"`, ... is not the same as `"a"`, `"b"`, `"c"`, .... This means that if a user types `"Yes"` it will not work and the user will redo the `repeat` loop even though the user typed `"Yes"`. This is probably not what we want...

Luckily, Lua has functions that allow us to convert a string to all uppercase or all lowercase: `string.upper` and `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

Which allows us to do the following

<div class="runner">

```lua
repeat 
	print("Do you want to continue? yes / no")
	user_answer = string.lower(io.read())
	if user_answer ~= "yes" and user_answer ~= "no" then
		print("This is not a valid answer.")
	end
until user_answer == "yes" or user_answer == "no"
```

</div>

With this, the `user_answer` will always be in lowercase due to `string.lower()`.
We also add a check just to print a message if the response is neither `"yes"` nor `"no"`. 
Which gives us this full program:

<div class="runner">

```lua
repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("Try to guess the random number between 1 and 100!")

	repeat
		repeat
			user_guess = tonumber(io.read())
			if user_guess == nil then 
				print("This is not a valid number!")
			end
		until user_guess ~= nil
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("The number is lower!")
		end

		if number_to_guess > user_guess then
			print("The number is higher!")
		end
	until user_guess == number_to_guess

	print("You won in " .. number_of_tries .. " tries!")

	repeat 
		print("Do you want to continue? yes / no")
		user_answer = string.lower(io.read())
		if user_answer ~= "yes" and user_answer ~= "no" then
			print("This is not a valid answer.")
		end
	until user_answer == "yes" or user_answer == "no"
until user_answer == "no"

print("Goodbye!")
```

</div>
