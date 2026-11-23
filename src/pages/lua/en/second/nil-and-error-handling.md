The first thing that you may have noticed is that our game currently *crashes* when we enter a number that isn't a valid number: 

<div class="runner">

```lua
user_guess = tonumber(io.read())

if 10 < user_guess then
	print("The number is lower!")
end
```

</div>

which, if you don't type a number will give you:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

But what does this mean?

The core concept here is `nil`. While introducing the Lua types, we briefly saw the `nil` type:

> `nil`: Empty. The goal of `nil` is to say that you don't have a value.

<div style="height: 3px"></div>

And this is _exactly_ what `tonumber` is doing:

- If `io.read()` is a string that represents a number like `"3.14"` or `"42"`, Lua knows how to convert it from a `string` type to a `number` type.
- Else, if `io.read()` is a string that doesn't look like a `number`, `tonumber` just says "I don't know how to convert this to a number, let's just return `nil`".

Therefore, if in `io.read()` we don't pass a correct representation of a number, we will just have `nil`:

<div class="runner">

```lua
user_guess = tonumber("123abc")
print(user_guess)
```

</div>

## Ok, but why does it create an error then?

Good question! For now, the `user_guess` is just `nil`. The error occurs when we try to compare with the `<` operator, a `number` to `nil` with this line:

```lua
if 10 < user_guess then
```

And the error is well described in the error message:

```
Error: [string "user_guess = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` means the line at which the error occurs, in this case, our line of comparing.
- `attempt to compare number with nil` is the reason why there has been an error.

And this makes perfect sense! What does it mean to compare, in terms of greater or less, nothing with a number? Is it `false`? Is it `true`? No, the question just doesn't make any sense. So Lua just errors.

## Handling the error
The quickest way to handle this is just to check if `user_guess` is `nil` and this can be done with the `==` operator:

```lua
if user_guess == nil then
	print("This is not a valid number")
end
```

But in this case, we don't want to just print something and continue the loop because it would count it as a guess because of:

```lua
number_of_tries = number_of_tries + 1
```

An easy way to do this would be to ask for `user_guess` until it's not `nil` with the `~=` operator, meaning, until it's a valid `number`:

```lua
repeat
	user_guess = tonumber(io.read())
until user_guess ~= nil
```

We can also add an error message in case it is with:

```lua
repeat
	user_guess = tonumber(io.read())
	if user_guess == nil then 
		print("This is not a valid number!")
	end
until user_guess ~= nil
```

Which gives us this full program:


<div class="runner">

```lua
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
```

</div>
