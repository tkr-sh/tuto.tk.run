So, now the only thing that remains is printing the number of tries.

We previously incremented it just after getting the user data with

```lua
number_of_tries = number_of_tries + 1
```

But know, we would like to print a congratulation message and at the same time, how many tries did it take ?

It's time to learn the last operator before the end of this first game:

`..`: Concatenates 2 values. 

<div class="runner">

```lua
print("It's been " .. 4 .. " years")
```

</div>

With that, in mind, we can create a nice message:

```lua
print("You won in " .. number_of_tries .. " tries!")
```

And here is FINALLY our game:

---

<div class="runner">

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

--- 

## WOW.

Wow! Isn't it ?

You're ready to become a programmer now :) !

## Is it over ?

Yes and no.

Sadly, I oversimplified a thing: usually you can't execute `read` and `random` like that. Lua likes to put "built-in"s function in some categories: `math` (for mathematicals functions), `io` (for input / output interactions), `os` (to interact with your operating system), ...

And therefore, when you want to use specific functions that are part of a category, you need to call them like `category.the_function()`

In our case, `random` is part of the `math` category, and `read` is part of the `io` category. `print` and `tonumber` are global so you don't need to specify anything

<div class="runner">

```lua
number_of_tries = 0
number_to_guess = math.random(1, 100)

print("Try to guess the random number between 1 and 100!")

repeat
	user_guess = tonumber(io.read())
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

## Congratulations, you have now finished your first Lua program! 🎉

Now that you have the basis, we will try to improve our current game to add new functionnalities, and we are going to learn new things on the way!
